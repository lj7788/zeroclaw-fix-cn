use super::{IntegrationCategory, IntegrationEntry, IntegrationStatus};
use crate::providers::{
    is_glm_alias, is_minimax_alias, is_moonshot_alias, is_qianfan_alias, is_qwen_alias,
    is_zai_alias,
};

/// Returns the full catalog of integrations
#[allow(clippy::too_many_lines)]
pub fn all_integrations() -> Vec<IntegrationEntry> {
    vec![
        // ── Chat Providers ──────────────────────────────────────
        IntegrationEntry {
            name: "Telegram",
            description: "机器人 API — 长轮询",
            category: IntegrationCategory::Chat,
            status_fn: |c| {
                if c.channels_config.telegram.is_some() {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Discord",
            description: "服务器、频道和私信",
            category: IntegrationCategory::Chat,
            status_fn: |c| {
                if c.channels_config.discord.is_some() {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Slack",
            description: "通过 Web API 的工作区应用",
            category: IntegrationCategory::Chat,
            status_fn: |c| {
                if c.channels_config.slack.is_some() {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Webhooks",
            description: "触发器的 HTTP 端点",
            category: IntegrationCategory::Chat,
            status_fn: |c| {
                if c.channels_config.webhook.is_some() {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "WhatsApp",
            description: "通过 webhook 的 Meta Cloud API",
            category: IntegrationCategory::Chat,
            status_fn: |c| {
                if c.channels_config.whatsapp.is_some() {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Signal",
            description: "通过 signal-cli 的隐私优先",
            category: IntegrationCategory::Chat,
            status_fn: |c| {
                if c.channels_config.signal.is_some() {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "iMessage",
            description: "macOS AppleScript 桥接",
            category: IntegrationCategory::Chat,
            status_fn: |c| {
                if c.channels_config.imessage.is_some() {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Microsoft Teams",
            description: "企业聊天支持",
            category: IntegrationCategory::Chat,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Matrix",
            description: "Matrix 协议（Element）",
            category: IntegrationCategory::Chat,
            status_fn: |c| {
                if c.channels_config.matrix.is_some() {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Nostr",
            description: "去中心化私信（NIP-04）",
            category: IntegrationCategory::Chat,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "WebChat",
            description: "基于浏览器的聊天界面",
            category: IntegrationCategory::Chat,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Nextcloud Talk",
            description: "自托管的 Nextcloud 聊天",
            category: IntegrationCategory::Chat,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Zalo",
            description: "Zalo 机器人 API",
            category: IntegrationCategory::Chat,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "DingTalk",
            description: "钉钉流模式",
            category: IntegrationCategory::Chat,
            status_fn: |c| {
                if c.channels_config.dingtalk.is_some() {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "QQ Official",
            description: "腾讯 QQ 机器人 SDK",
            category: IntegrationCategory::Chat,
            status_fn: |c| {
                if c.channels_config.qq.is_some() {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        // ── AI Models ───────────────────────────────────────────
        IntegrationEntry {
            name: "OpenRouter",
            description: "Claude Sonnet 4.6、GPT-5.2、Gemini 3.1 Pro",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref() == Some("openrouter") && c.api_key.is_some() {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Anthropic",
            description: "Claude Sonnet 4.6、Claude Opus 4.6",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref() == Some("anthropic") {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "OpenAI",
            description: "GPT-5.2、GPT-5.2-Codex",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref() == Some("openai") {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Google",
            description: "Gemini 3.1 Pro、Gemini 3 Flash",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_model
                    .as_deref()
                    .is_some_and(|m| m.starts_with("google/"))
                {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "DeepSeek",
            description: "DeepSeek-Reasoner、DeepSeek-Chat",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_model
                    .as_deref()
                    .is_some_and(|m| m.starts_with("deepseek/"))
                {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "xAI",
            description: "Grok 4、Grok 3",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_model
                    .as_deref()
                    .is_some_and(|m| m.starts_with("x-ai/"))
                {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Mistral",
            description: "Mistral Large 最新版、Codestral",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_model
                    .as_deref()
                    .is_some_and(|m| m.starts_with("mistral"))
                {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Ollama",
            description: "本地模型（Llama 等）",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref() == Some("ollama") {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Perplexity",
            description: "Sonar Pro、Sonar Reasoning Pro",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref() == Some("perplexity") {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Hugging Face",
            description: "开源模型",
            category: IntegrationCategory::AiModel,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "LM Studio",
            description: "本地模型服务器",
            category: IntegrationCategory::AiModel,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Venice",
            description: "Venice Llama 3.3 70B 和前沿混合模型",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref() == Some("venice") {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Vercel AI",
            description: "GPT-5.2 和多提供商路由的网关",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref() == Some("vercel") {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Cloudflare AI",
            description: "Workers AI + Llama 3.3 / 网关路由",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref() == Some("cloudflare") {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Moonshot",
            description: "Kimi 2.5 和 Kimi Coding",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref().is_some_and(is_moonshot_alias) {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Synthetic",
            description: "Synthetic-1 和 synthetic 系列模型",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref() == Some("synthetic") {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "OpenCode Zen",
            description: "OpenCode Zen 和编码专用模型",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref() == Some("opencode") {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Z.AI",
            description: "GLM 4.7 和 Z.AI 托管变体",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref().is_some_and(is_zai_alias) {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "GLM",
            description: "GLM 4.7 和 GLM 4.5 系列",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref().is_some_and(is_glm_alias) {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "MiniMax",
            description: "MiniMax M1 和最新的多模态变体",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref().is_some_and(is_minimax_alias) {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Qwen",
            description: "Qwen Max 和 Qwen 推理系列",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref().is_some_and(is_qwen_alias) {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Amazon Bedrock",
            description: "Claude Sonnet 4.5 和 Bedrock 模型目录",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref() == Some("bedrock") {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Qianfan",
            description: "ERNIE 4.x 和千帆模型目录",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref().is_some_and(is_qianfan_alias) {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Groq",
            description: "Llama 3.3 70B 多功能和低延迟模型",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref() == Some("groq") {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Together AI",
            description: "Llama 3.3 70B Turbo 和开源模型托管",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref() == Some("together") {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Fireworks AI",
            description: "DeepSeek / Llama 高吞吐量推理",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref() == Some("fireworks") {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Cohere",
            description: "Command R+ (2024年8月) 和嵌入模型",
            category: IntegrationCategory::AiModel,
            status_fn: |c| {
                if c.default_provider.as_deref() == Some("cohere") {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        // ── Productivity ────────────────────────────────────────
        IntegrationEntry {
            name: "GitHub",
            description: "代码、问题、PR",
            category: IntegrationCategory::Productivity,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Notion",
            description: "工作区和数据库",
            category: IntegrationCategory::Productivity,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Apple Notes",
            description: "原生 macOS/iOS 笔记",
            category: IntegrationCategory::Productivity,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Apple Reminders",
            description: "任务管理",
            category: IntegrationCategory::Productivity,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Obsidian",
            description: "知识图谱笔记",
            category: IntegrationCategory::Productivity,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Things 3",
            description: "GTD 任务管理器",
            category: IntegrationCategory::Productivity,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Bear Notes",
            description: "Markdown 笔记",
            category: IntegrationCategory::Productivity,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Trello",
            description: "看板",
            category: IntegrationCategory::Productivity,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Linear",
            description: "问题跟踪",
            category: IntegrationCategory::Productivity,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        // ── Music & Audio ───────────────────────────────────────
        IntegrationEntry {
            name: "Spotify",
            description: "音乐播放控制",
            category: IntegrationCategory::MusicAudio,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Sonos",
            description: "多房间音频",
            category: IntegrationCategory::MusicAudio,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Shazam",
            description: "歌曲识别",
            category: IntegrationCategory::MusicAudio,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        // ── Smart Home ──────────────────────────────────────────
        IntegrationEntry {
            name: "Home Assistant",
            description: "家庭自动化中心",
            category: IntegrationCategory::SmartHome,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Philips Hue",
            description: "智能照明",
            category: IntegrationCategory::SmartHome,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "8Sleep",
            description: "智能床垫",
            category: IntegrationCategory::SmartHome,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        // ── Tools & Automation ──────────────────────────────────
        IntegrationEntry {
            name: "Browser",
            description: "Chrome/Chromium 控制",
            category: IntegrationCategory::ToolsAutomation,
            status_fn: |_| IntegrationStatus::Available,
        },
        IntegrationEntry {
            name: "Shell",
            description: "终端命令执行",
            category: IntegrationCategory::ToolsAutomation,
            status_fn: |_| IntegrationStatus::Active,
        },
        IntegrationEntry {
            name: "File System",
            description: "读写文件",
            category: IntegrationCategory::ToolsAutomation,
            status_fn: |_| IntegrationStatus::Active,
        },
        IntegrationEntry {
            name: "Cron",
            description: "计划任务",
            category: IntegrationCategory::ToolsAutomation,
            status_fn: |_| IntegrationStatus::Available,
        },
        IntegrationEntry {
            name: "Voice",
            description: "语音唤醒 + 对话模式",
            category: IntegrationCategory::ToolsAutomation,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Gmail",
            description: "邮件触发器和发送",
            category: IntegrationCategory::ToolsAutomation,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "1Password",
            description: "安全凭证",
            category: IntegrationCategory::ToolsAutomation,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Weather",
            description: "天气预报和状况",
            category: IntegrationCategory::ToolsAutomation,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Canvas",
            description: "可视化工作区 + A2UI",
            category: IntegrationCategory::ToolsAutomation,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        // ── Media & Creative ────────────────────────────────────
        IntegrationEntry {
            name: "Image Gen",
            description: "AI 图像生成",
            category: IntegrationCategory::MediaCreative,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "GIF Search",
            description: "查找完美的 GIF",
            category: IntegrationCategory::MediaCreative,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Screen Capture",
            description: "截图和屏幕控制",
            category: IntegrationCategory::MediaCreative,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Camera",
            description: "照片/视频捕获",
            category: IntegrationCategory::MediaCreative,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        // ── Social ──────────────────────────────────────────────
        IntegrationEntry {
            name: "Twitter/X",
            description: "发推文、回复、搜索",
            category: IntegrationCategory::Social,
            status_fn: |_| IntegrationStatus::ComingSoon,
        },
        IntegrationEntry {
            name: "Email",
            description: "IMAP/SMTP 邮件频道",
            category: IntegrationCategory::Social,
            status_fn: |c| {
                if c.channels_config.email.is_some() {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        // ── Platforms ───────────────────────────────────────────
        IntegrationEntry {
            name: "macOS",
            description: "原生支持 + AppleScript",
            category: IntegrationCategory::Platform,
            status_fn: |_| {
                if cfg!(target_os = "macos") {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Linux",
            description: "原生支持",
            category: IntegrationCategory::Platform,
            status_fn: |_| {
                if cfg!(target_os = "linux") {
                    IntegrationStatus::Active
                } else {
                    IntegrationStatus::Available
                }
            },
        },
        IntegrationEntry {
            name: "Windows",
            description: "推荐 WSL2",
            category: IntegrationCategory::Platform,
            status_fn: |_| IntegrationStatus::Available,
        },
        IntegrationEntry {
            name: "iOS",
            description: "通过 Telegram/Discord 聊天",
            category: IntegrationCategory::Platform,
            status_fn: |_| IntegrationStatus::Available,
        },
        IntegrationEntry {
            name: "Android",
            description: "通过 Telegram/Discord 聊天",
            category: IntegrationCategory::Platform,
            status_fn: |_| IntegrationStatus::Available,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::{IMessageConfig, MatrixConfig, StreamMode, TelegramConfig};
    use crate::config::Config;

    #[test]
    fn registry_has_entries() {
        let entries = all_integrations();
        assert!(
            entries.len() >= 50,
            "Expected 50+ integrations, got {}",
            entries.len()
        );
    }

    #[test]
    fn all_categories_represented() {
        let entries = all_integrations();
        for cat in IntegrationCategory::all() {
            let count = entries.iter().filter(|e| e.category == *cat).count();
            assert!(count > 0, "Category {cat:?} has no entries");
        }
    }

    #[test]
    fn status_functions_dont_panic() {
        let config = Config::default();
        let entries = all_integrations();
        for entry in &entries {
            let _ = (entry.status_fn)(&config);
        }
    }

    #[test]
    fn no_duplicate_names() {
        let entries = all_integrations();
        let mut seen = std::collections::HashSet::new();
        for entry in &entries {
            assert!(
                seen.insert(entry.name),
                "Duplicate integration name: {}",
                entry.name
            );
        }
    }

    #[test]
    fn no_empty_names_or_descriptions() {
        let entries = all_integrations();
        for entry in &entries {
            assert!(!entry.name.is_empty(), "Found integration with empty name");
            assert!(
                !entry.description.is_empty(),
                "Integration '{}' has empty description",
                entry.name
            );
        }
    }

    #[test]
    fn telegram_active_when_configured() {
        let mut config = Config::default();
        config.channels_config.telegram = Some(TelegramConfig {
            bot_token: "123:ABC".into(),
            allowed_users: vec!["user".into()],
            stream_mode: StreamMode::default(),
            draft_update_interval_ms: 1000,
            interrupt_on_new_message: false,
            mention_only: false,
            group_reply: None,
            base_url: None,
        });
        let entries = all_integrations();
        let tg = entries.iter().find(|e| e.name == "Telegram").unwrap();
        assert!(matches!((tg.status_fn)(&config), IntegrationStatus::Active));
    }

    #[test]
    fn telegram_available_when_not_configured() {
        let config = Config::default();
        let entries = all_integrations();
        let tg = entries.iter().find(|e| e.name == "Telegram").unwrap();
        assert!(matches!(
            (tg.status_fn)(&config),
            IntegrationStatus::Available
        ));
    }

    #[test]
    fn imessage_active_when_configured() {
        let mut config = Config::default();
        config.channels_config.imessage = Some(IMessageConfig {
            allowed_contacts: vec!["*".into()],
        });
        let entries = all_integrations();
        let im = entries.iter().find(|e| e.name == "iMessage").unwrap();
        assert!(matches!((im.status_fn)(&config), IntegrationStatus::Active));
    }

    #[test]
    fn imessage_available_when_not_configured() {
        let config = Config::default();
        let entries = all_integrations();
        let im = entries.iter().find(|e| e.name == "iMessage").unwrap();
        assert!(matches!(
            (im.status_fn)(&config),
            IntegrationStatus::Available
        ));
    }

    #[test]
    fn matrix_active_when_configured() {
        let mut config = Config::default();
        config.channels_config.matrix = Some(MatrixConfig {
            homeserver: "https://m.org".into(),
            access_token: "tok".into(),
            user_id: None,
            device_id: None,
            room_id: "!r:m".into(),
            allowed_users: vec![],
            mention_only: false,
        });
        let entries = all_integrations();
        let mx = entries.iter().find(|e| e.name == "Matrix").unwrap();
        assert!(matches!((mx.status_fn)(&config), IntegrationStatus::Active));
    }

    #[test]
    fn matrix_available_when_not_configured() {
        let config = Config::default();
        let entries = all_integrations();
        let mx = entries.iter().find(|e| e.name == "Matrix").unwrap();
        assert!(matches!(
            (mx.status_fn)(&config),
            IntegrationStatus::Available
        ));
    }

    #[test]
    fn coming_soon_integrations_stay_coming_soon() {
        let config = Config::default();
        let entries = all_integrations();
        for name in ["Nostr", "Spotify", "Home Assistant"] {
            let entry = entries.iter().find(|e| e.name == name).unwrap();
            assert!(
                matches!((entry.status_fn)(&config), IntegrationStatus::ComingSoon),
                "{name} should be ComingSoon"
            );
        }
    }

    #[test]
    fn whatsapp_available_when_not_configured() {
        let config = Config::default();
        let entries = all_integrations();
        let wa = entries.iter().find(|e| e.name == "WhatsApp").unwrap();
        assert!(matches!(
            (wa.status_fn)(&config),
            IntegrationStatus::Available
        ));
    }

    #[test]
    fn email_available_when_not_configured() {
        let config = Config::default();
        let entries = all_integrations();
        let email = entries.iter().find(|e| e.name == "Email").unwrap();
        assert!(matches!(
            (email.status_fn)(&config),
            IntegrationStatus::Available
        ));
    }

    #[test]
    fn shell_and_filesystem_always_active() {
        let config = Config::default();
        let entries = all_integrations();
        for name in ["Shell", "File System"] {
            let entry = entries.iter().find(|e| e.name == name).unwrap();
            assert!(
                matches!((entry.status_fn)(&config), IntegrationStatus::Active),
                "{name} should always be Active"
            );
        }
    }

    #[test]
    fn macos_active_on_macos() {
        let config = Config::default();
        let entries = all_integrations();
        let macos = entries.iter().find(|e| e.name == "macOS").unwrap();
        let status = (macos.status_fn)(&config);
        if cfg!(target_os = "macos") {
            assert!(matches!(status, IntegrationStatus::Active));
        } else {
            assert!(matches!(status, IntegrationStatus::Available));
        }
    }

    #[test]
    fn category_counts_reasonable() {
        let entries = all_integrations();
        let chat_count = entries
            .iter()
            .filter(|e| e.category == IntegrationCategory::Chat)
            .count();
        let ai_count = entries
            .iter()
            .filter(|e| e.category == IntegrationCategory::AiModel)
            .count();
        assert!(
            chat_count >= 5,
            "Expected 5+ chat integrations, got {chat_count}"
        );
        assert!(
            ai_count >= 5,
            "Expected 5+ AI model integrations, got {ai_count}"
        );
    }

    #[test]
    fn regional_provider_aliases_activate_expected_ai_integrations() {
        let entries = all_integrations();
        let mut config = Config {
            default_provider: Some("minimax-cn".to_string()),
            ..Config::default()
        };

        let minimax = entries.iter().find(|e| e.name == "MiniMax").unwrap();
        assert!(matches!(
            (minimax.status_fn)(&config),
            IntegrationStatus::Active
        ));

        config.default_provider = Some("glm-cn".to_string());
        let glm = entries.iter().find(|e| e.name == "GLM").unwrap();
        assert!(matches!(
            (glm.status_fn)(&config),
            IntegrationStatus::Active
        ));

        config.default_provider = Some("moonshot-intl".to_string());
        let moonshot = entries.iter().find(|e| e.name == "Moonshot").unwrap();
        assert!(matches!(
            (moonshot.status_fn)(&config),
            IntegrationStatus::Active
        ));

        config.default_provider = Some("qwen-intl".to_string());
        let qwen = entries.iter().find(|e| e.name == "Qwen").unwrap();
        assert!(matches!(
            (qwen.status_fn)(&config),
            IntegrationStatus::Active
        ));

        config.default_provider = Some("zai-cn".to_string());
        let zai = entries.iter().find(|e| e.name == "Z.AI").unwrap();
        assert!(matches!(
            (zai.status_fn)(&config),
            IntegrationStatus::Active
        ));

        config.default_provider = Some("baidu".to_string());
        let qianfan = entries.iter().find(|e| e.name == "Qianfan").unwrap();
        assert!(matches!(
            (qianfan.status_fn)(&config),
            IntegrationStatus::Active
        ));
    }
}
