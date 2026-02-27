//! CLI Internationalization utilities
//!
//! Provides translation functions for command-line interface messages.

use std::collections::HashMap;

/// Supported locales
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locale {
    En,
    ZhCN,
}

/// Translation function that returns the translated string
pub fn t(key: &str) -> String {
    // Simple implementation - in a real i18n system, this would look up translations
    // For now, we'll just return the key or a default Chinese translation
    let translations: HashMap<&str, &str> = [
        ("cli.about", "ZeroClaw - 零开销，零妥协，100% Rust"),
        ("common.loading", "加载中..."),
        ("common.error", "发生错误。"),
    ].iter().cloned().collect();
    
    translations.get(key).map(|s| s.to_string()).unwrap_or_else(|| key.to_string())
}

/// CLI translations structure
pub struct CliTranslations;

impl CliTranslations {
    pub fn new() -> Self {
        Self
    }
    
    pub fn get(&self, key: &str) -> String {
        t(key)
    }
}