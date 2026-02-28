use crate::providers::{ChatMessage, ChatResponse, ConversationMessage, ToolResultMessage};
use crate::tools::{Tool, ToolSpec};
use serde_json::Value;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct ParsedToolCall {
    pub name: String,
    pub arguments: Value,
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ToolExecutionResult {
    pub name: String,
    pub output: String,
    pub success: bool,
    pub tool_call_id: Option<String>,
}

pub trait ToolDispatcher: Send + Sync {
    fn parse_response(&self, response: &ChatResponse) -> (String, Vec<ParsedToolCall>);
    fn format_results(&self, results: &[ToolExecutionResult]) -> ConversationMessage;
    fn prompt_instructions(&self, tools: &[Box<dyn Tool>]) -> String;
    fn to_provider_messages(&self, history: &[ConversationMessage]) -> Vec<ChatMessage>;
    fn should_send_tool_specs(&self) -> bool;
}

#[derive(Default)]
pub struct XmlToolDispatcher;

impl XmlToolDispatcher {
    fn parse_xml_tool_calls(response: &str) -> (String, Vec<ParsedToolCall>) {
        let mut text_parts = Vec::new();
        let mut calls = Vec::new();
        let remaining = response;

        let tag_patterns = [
            ("<tool_call>", "</tool_call>"),
            ("<toolcall>", "</toolcall>"),
            ("<tool-call>", "</tool-call>"),
            ("<invoke>", "</invoke>"),
            ("<poetry>", "</poetry>"),
            ("<poetry_write", ">"),
            ("<poem_write", ">"),
            ("<poetry_call", ">"),
            ("<poetry_tool_call", ">"),
            ("<output>", "</output>"),
            ("<poetry_call>", "</poetry_call>"),
            ("<poetry_tool_call>", "</poetry_tool_call>"),
            ("<poem_write>", "</poem_write>"),
            ("<trash>", "</trash>"),
            ("<tool_call", ">"),
            ("<poem>", "</poem>"),
            ("<poem_call>", "</poem_call>"),
            ("<poem_tool_call>", "</poem_tool_call>"),
            ("<poem_generator>", "</poem_generator>"),
            ("<poem_writer>", "</poem_writer>"),
            ("<poetry_writer>", "</poetry_writer>"),
            ("<poem_create>", "</poem_create>"),
            ("<poetry_create>", "</poetry_create>"),
            ("<poem_generate>", "</poem_generate>"),
            ("<poetry_generate>", "</poetry_generate>"),
            ("<poem_output>", "</poem_output>"),
            ("<poetry_output>", "</poetry_output>"),
            ("<poem_result>", "</poem_result>"),
            ("<poetry_result>", "</poetry_result>"),
            ("<poem_response>", "</poem_response>"),
            ("<poetry_response>", "</poetry_response>"),
            ("<poem_text>", "</poem_text>"),
            ("<poetry_text>", "</poetry_text>"),
            ("<poem_content>", "</poem_content>"),
            ("<poetry_content>", "</poetry_content>"),
            // TTS tool aliases
            ("<text_to_speech>", "</text_to_speech>"),
            ("<text_to_speech", ">"),
            ("<voice_say>", "</voice_say>"),
            ("<voice_say", ">"),
            ("<speak>", "</speak>"),
            ("<speak", ">"),
            ("<say>", "</say>"),
            ("<say", ">"),
            ("<tts>", "</tts>"),
            ("<tts", ">"),
        ];

        let mut current = remaining;

        while !current.is_empty() {
            let mut found = false;
            for (open_tag, close_tag) in &tag_patterns {
                if let Some(start) = current.find(open_tag) {
                    if start > 0 {
                        let before = &current[..start];
                        if !before.trim().is_empty() {
                            text_parts.push(before.trim().to_string());
                        }
                    }

                    // Try to find closing tag, if not found, use rest of string
                    let (end_pos, inner) = if let Some(end) = current[start..].find(close_tag) {
                        let end_pos = start + end + close_tag.len();
                        let inner = &current[start + open_tag.len()..start + end];
                        (end_pos, inner)
                    } else {
                        // No closing tag found - use rest of string as content
                        let end_pos = current.len();
                        let inner = &current[start + open_tag.len()..end_pos];
                        (end_pos, inner)
                    };

                    if open_tag.starts_with("<tool") || *open_tag == "<invoke>" {
                            // First try to parse as standard JSON format: {"name": "...", "arguments": {...}}
                            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(inner.trim()) {
                                let name = parsed.get("name").and_then(serde_json::Value::as_str).unwrap_or("").to_string();
                                if !name.is_empty() {
                                    let arguments = parsed.get("arguments").cloned().unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new()));
                                    calls.push(ParsedToolCall {
                                        name,
                                        arguments,
                                        tool_call_id: None,
                                    });
                                } else {
                                    text_parts.push(inner.trim().to_string());
                                }
                            } else {
                                // Try to parse format: tool_name\n{"arg": "value"} or tool_name\narg=value\narg2=value2
                                let inner_trimmed = inner.trim();
                                if let Some(first_line_end) = inner_trimmed.find('\n') {
                                    let first_line = &inner_trimmed[..first_line_end].trim();
                                    let rest = &inner_trimmed[first_line_end..].trim();
                                    
                                    // Check if first line is a tool name (not JSON)
                                    if !first_line.is_empty() && !first_line.starts_with('{') {
                                        let tool_name = first_line.to_string();
                                        // Try to parse the rest as JSON arguments first
                                        let arguments = if let Ok(args) = serde_json::from_str::<serde_json::Value>(rest) {
                                            args
                                        } else {
                                            // Try to parse as key=value lines
                                            let mut args_map = serde_json::Map::new();
                                            for line in rest.lines() {
                                                let line_trimmed = line.trim();
                                                if let Some((key, value)) = line_trimmed.split_once('=') {
                                                    let value = value.trim().trim_matches('"').trim_matches('\'');
                                                    args_map.insert(key.trim().to_string(), serde_json::Value::String(value.to_string()));
                                                }
                                            }
                                            serde_json::Value::Object(args_map)
                                        };
                                        calls.push(ParsedToolCall {
                                            name: tool_name,
                                            arguments,
                                            tool_call_id: None,
                                        });
                                    } else {
                                        text_parts.push(inner.trim().to_string());
                                    }
                                } else {
                                    // Single line content, check if it's a tool name
                                    let tag_content = open_tag.trim_start_matches("<").trim_end_matches(">");
                                    let mut parts = tag_content.split_whitespace();
                                    if let Some(name) = parts.next() {
                                        if name.starts_with("tool") || name == "invoke" {
                                            let mut arguments = serde_json::Map::new();
                                            for part in parts {
                                                if let Some((key, value)) = part.split_once('=') {
                                                    let value = value.trim_matches('"');
                                                    arguments.insert(key.to_string(), serde_json::Value::String(value.to_string()));
                                                }
                                            }
                                            calls.push(ParsedToolCall {
                                                name: name.to_string(),
                                                arguments: serde_json::Value::Object(arguments),
                                                tool_call_id: None,
                                            });
                                        } else {
                                            text_parts.push(inner.trim().to_string());
                                        }
                                    } else {
                                        text_parts.push(inner.trim().to_string());
                                    }
                                }
                            }
                        } else if open_tag.starts_with("<poetry") || open_tag.starts_with("<poem") || *open_tag == "<output>" || *open_tag == "<trash>" {
                            text_parts.push(inner.trim().to_string());
                        } else if open_tag.starts_with("<text_to_speech") || open_tag.starts_with("<voice_say") || 
                                  open_tag.starts_with("<speak") || open_tag.starts_with("<say") || open_tag.starts_with("<tts") {
                            // Handle TTS tool tags - extract attributes from tag and content
                            let tag_content = open_tag.trim_start_matches('<').trim_end_matches('>');
                            let mut parts = tag_content.split_whitespace();
                            let tag_name = parts.next().unwrap_or("");
                            
                            // Map tag name to tool name
                            let tool_name = match tag_name {
                                "text_to_speech" | "tts" => "tts",
                                "voice_say" => "tts",
                                "speak" | "say" => "tts",
                                _ => "tts",
                            };
                            
                            let mut arguments = serde_json::Map::new();
                            
                            // Parse attributes from tag
                            for part in parts {
                                if let Some((key, value)) = part.split_once('=') {
                                    let value = value.trim_matches('"');
                                    arguments.insert(key.to_string(), serde_json::Value::String(value.to_string()));
                                }
                            }
                            
                            // If inner content exists and text is not already set, use it as text
                            let inner_trimmed = inner.trim();
                            if !inner_trimmed.is_empty() && !arguments.contains_key("text") {
                                arguments.insert("text".to_string(), serde_json::Value::String(inner_trimmed.to_string()));
                            }
                            
                            calls.push(ParsedToolCall {
                                name: tool_name.to_string(),
                                arguments: serde_json::Value::Object(arguments),
                                tool_call_id: None,
                            });
                    } else {
                        text_parts.push(inner.trim().to_string());
                    }

                    current = &current[end_pos..];
                    found = true;
                    break;
                }
            }

            if !found {
                if !current.trim().is_empty() {
                    text_parts.push(current.trim().to_string());
                }
                break;
            }
        }

        (text_parts.join("\n"), calls)
    }

    pub fn tool_specs(tools: &[Box<dyn Tool>]) -> Vec<ToolSpec> {
        tools.iter().map(|tool| tool.spec()).collect()
    }
}

impl ToolDispatcher for XmlToolDispatcher {
    fn parse_response(&self, response: &ChatResponse) -> (String, Vec<ParsedToolCall>) {
        let text = response.text_or_empty();
        Self::parse_xml_tool_calls(text)
    }

    fn format_results(&self, results: &[ToolExecutionResult]) -> ConversationMessage {
        let mut content = String::new();
        for result in results {
            let status = if result.success { "ok" } else { "error" };
            let _ = writeln!(
                content,
                "<tool_result name=\"{}\" status=\"{}\">\n{}\n</tool_result>",
                result.name, status, result.output
            );
        }
        ConversationMessage::Chat(ChatMessage::user(format!("[Tool results]\n{content}")))
    }

    fn prompt_instructions(&self, tools: &[Box<dyn Tool>]) -> String {
        let mut instructions = String::new();
        instructions.push_str("## Tool Use Protocol\n\n");
        instructions
            .push_str("To use a tool, wrap a JSON object in <tool_call></tool_call> tags:\n\n");
        instructions.push_str(
            "```\n<tool_call>\n{\"name\": \"tool_name\", \"arguments\": {\"param\": \"value\"}}\n</tool_call>\n```\n\n",
        );
        instructions.push_str("### Available Tools\n\n");

        for tool in tools {
            let _ = writeln!(
                instructions,
                "- **{}**: {}\n  Parameters: `{}`",
                tool.name(),
                tool.description(),
                tool.parameters_schema()
            );
        }

        instructions
    }

    fn to_provider_messages(&self, history: &[ConversationMessage]) -> Vec<ChatMessage> {
        history
            .iter()
            .flat_map(|msg| match msg {
                ConversationMessage::Chat(chat) => vec![chat.clone()],
                ConversationMessage::AssistantToolCalls { text, .. } => {
                    vec![ChatMessage::assistant(text.clone().unwrap_or_default())]
                }
                ConversationMessage::ToolResults(results) => {
                    let mut content = String::new();
                    for result in results {
                        let _ = writeln!(
                            content,
                            "<tool_result id=\"{}\">\n{}\n</tool_result>",
                            result.tool_call_id, result.content
                        );
                    }
                    vec![ChatMessage::user(format!("[Tool results]\n{content}"))]
                }
            })
            .collect()
    }

    fn should_send_tool_specs(&self) -> bool {
        false
    }
}

pub struct NativeToolDispatcher;

impl ToolDispatcher for NativeToolDispatcher {
    fn parse_response(&self, response: &ChatResponse) -> (String, Vec<ParsedToolCall>) {
        let text = response.text.clone().unwrap_or_default();
        let calls = response
            .tool_calls
            .iter()
            .map(|tc| ParsedToolCall {
                name: tc.name.clone(),
                arguments: serde_json::from_str(&tc.arguments).unwrap_or_else(|e| {
                    tracing::warn!(
                        tool = %tc.name,
                        error = %e,
                        "Failed to parse native tool call arguments as JSON; defaulting to empty object"
                    );
                    Value::Object(serde_json::Map::new())
                }),
                tool_call_id: Some(tc.id.clone()),
            })
            .collect();
        (text, calls)
    }

    fn format_results(&self, results: &[ToolExecutionResult]) -> ConversationMessage {
        let messages = results
            .iter()
            .map(|result| ToolResultMessage {
                tool_call_id: result
                    .tool_call_id
                    .clone()
                    .unwrap_or_else(|| "unknown".to_string()),
                content: result.output.clone(),
            })
            .collect();
        ConversationMessage::ToolResults(messages)
    }

    fn prompt_instructions(&self, _tools: &[Box<dyn Tool>]) -> String {
        String::new()
    }

    fn to_provider_messages(&self, history: &[ConversationMessage]) -> Vec<ChatMessage> {
        history
            .iter()
            .flat_map(|msg| match msg {
                ConversationMessage::Chat(chat) => vec![chat.clone()],
                ConversationMessage::AssistantToolCalls { text, tool_calls, .. } => {
                    let mut messages = Vec::new();
                    if let Some(text) = text {
                        messages.push(ChatMessage::assistant(text.clone()));
                    }
                    for tc in tool_calls {
                        messages.push(ChatMessage::assistant(format!("Tool call: {}", tc.name)));
                    }
                    messages
                }
                ConversationMessage::ToolResults(results) => {
                    let mut content = String::new();
                    for result in results {
                        let _ = writeln!(
                            content,
                            "Tool result for {}: {}",
                            result.tool_call_id, result.content
                        );
                    }
                    vec![ChatMessage::user(content)]
                }
            })
            .collect()
    }

    fn should_send_tool_specs(&self) -> bool {
        true
    }
}
