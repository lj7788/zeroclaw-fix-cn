use super::traits::{Tool, ToolResult};
use async_trait::async_trait;
use serde_json::json;
use std::time::Duration;

/// Gitee AI Text-to-Speech tool
/// Converts text to speech using Gitee AI's Spark-TTS model
pub struct GiteeTtsTool {
    api_token: String,
    timeout_secs: u64,
}

impl GiteeTtsTool {
    pub fn new(api_token: String) -> Self {
        Self {
            api_token,
            timeout_secs: 300, // 5 minutes default timeout for async TTS
        }
    }

    /// Create a TTS task
    async fn create_task(&self, text: &str, gender: &str, pitch: i32, speed: i32) -> anyhow::Result<String> {
        let client = reqwest::Client::new();
        let response = client
            .post("https://ai.gitee.com/v1/async/audio/speech")
            .header("Authorization", format!("Bearer {}", self.api_token))
            .json(&json!({
                "inputs": text,
                "model": "Spark-TTS-0.5B",
                "gender": gender,
                "pitch": pitch,
                "speed": speed
            }))
            .timeout(Duration::from_secs(30))
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        
        if let Some(error) = result.get("error") {
            anyhow::bail!("API error: {}", error);
        }

        let task_id = result
            .get("task_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Task ID not found in response"))?;

        Ok(task_id.to_string())
    }

    /// Poll task status until completion
    async fn poll_task(&self, task_id: &str) -> anyhow::Result<String> {
        let client = reqwest::Client::new();
        let status_url = format!("https://ai.gitee.com/v1/task/{}", task_id);
        let max_attempts = 180; // 180 * 10 seconds = 30 minutes max
        let retry_interval = Duration::from_secs(10);

        for _attempt in 1..=max_attempts {
            let response = client
                .get(&status_url)
                .header("Authorization", format!("Bearer {}", self.api_token))
                .timeout(Duration::from_secs(30))
                .send()
                .await?;

            let result: serde_json::Value = response.json().await?;

            if let Some(error) = result.get("error") {
                anyhow::bail!("Task error: {}", error);
            }

            let status = result
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");

            match status {
                "success" => {
                    let file_url = result
                        .get("output")
                        .and_then(|o| o.get("file_url"))
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| anyhow::anyhow!("File URL not found in successful response"))?;
                    return Ok(file_url.to_string());
                }
                "failed" | "cancelled" => {
                    anyhow::bail!("Task {}: {}", status, result.get("message").and_then(|v| v.as_str()).unwrap_or("Unknown error"));
                }
                _ => {
                    // Task is still processing, wait and retry
                    tokio::time::sleep(retry_interval).await;
                }
            }
        }

        anyhow::bail!("Task polling timeout after {} attempts", max_attempts)
    }
}

#[async_trait]
impl Tool for GiteeTtsTool {
    fn name(&self) -> &str {
        "tts"
    }

    fn description(&self) -> &str {
        "Text-to-Speech (TTS) - convert text to spoken audio. Input text content and get back a URL to download the generated audio file. Supports male/female voices and adjustable pitch/speed."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "text": {
                    "type": "string",
                    "description": "The text to convert to speech"
                },
                "gender": {
                    "type": "string",
                    "enum": ["male", "female"],
                    "description": "Voice gender (default: male)"
                },
                "pitch": {
                    "type": "integer",
                    "minimum": 1,
                    "maximum": 5,
                    "description": "Voice pitch level 1-5 (default: 3)"
                },
                "speed": {
                    "type": "integer",
                    "minimum": 1,
                    "maximum": 5,
                    "description": "Speech speed level 1-5 (default: 3)"
                }
            },
            "required": ["text"]
        })
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<ToolResult> {
        let text = args
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: text"))?;

        // Support both "gender" and "voice" parameters for flexibility
        let gender = args
            .get("gender")
            .or_else(|| args.get("voice"))
            .and_then(|v| v.as_str())
            .map(|v| {
                // Normalize voice/gender parameter
                let lower = v.to_lowercase();
                if lower.contains("female") || lower.contains("women") || lower.contains("girl") || lower.contains("女") {
                    "female"
                } else {
                    "male"
                }
            })
            .unwrap_or("male");

        let pitch = args
            .get("pitch")
            .and_then(|v| v.as_i64())
            .unwrap_or(3) as i32;

        let speed = args
            .get("speed")
            .and_then(|v| v.as_i64())
            .unwrap_or(3) as i32;

        // Validate parameters
        if text.is_empty() {
            return Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some("Text cannot be empty".to_string()),
            });
        }

        if text.len() > 5000 {
            return Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some("Text too long (max 5000 characters)".to_string()),
            });
        }

        // Create task
        let task_id = match self.create_task(text, gender, pitch, speed).await {
            Ok(id) => id,
            Err(e) => {
                return Ok(ToolResult {
                    success: false,
                    output: String::new(),
                    error: Some(format!("Failed to create TTS task: {}", e)),
                });
            }
        };

        // Poll for result
        match self.poll_task(&task_id).await {
            Ok(file_url) => {
                // Return structured JSON response for frontend audio player
                let json_output = serde_json::json!({
                    "type": "audio",
                    "url": file_url,
                    "text": text,
                    "message": "语音生成成功"
                });
                Ok(ToolResult {
                    success: true,
                    output: json_output.to_string(),
                    error: None,
                })
            }
            Err(e) => {
                Ok(ToolResult {
                    success: false,
                    output: String::new(),
                    error: Some(format!("TTS task failed: {}", e)),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gitee_tts_tool_name() {
        let tool = GiteeTtsTool::new("test_token".to_string());
        assert_eq!(tool.name(), "tts");
    }

    #[test]
    fn test_gitee_tts_tool_spec() {
        let tool = GiteeTtsTool::new("test_token".to_string());
        let spec = tool.spec();
        assert_eq!(spec.name, "tts");
        assert!(!spec.description.is_empty());
        assert!(spec.parameters.get("properties").is_some());
    }
}
