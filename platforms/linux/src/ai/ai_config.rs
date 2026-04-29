use serde::{Deserialize, Serialize};

/// Конфигурация ИИ чат-бота
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    /// API ключ
    pub api_key: String,
    /// Модель (например, "gpt-4", "gpt-3.5-turbo")
    pub model: String,
    /// Провайдер (openai, anthropic, local)
    pub provider: String,
    /// Максимальное количество токенов
    pub max_tokens: u32,
    /// Температура (0.0 - 2.0)
    pub temperature: f32,
    /// Системный промпт
    pub system_prompt: String,
    /// Включен ли ИИ чат-бот
    pub enabled: bool,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "gpt-3.5-turbo".to_string(),
            provider: "openai".to_string(),
            max_tokens: 1000,
            temperature: 0.7,
            system_prompt: "You are a helpful Minecraft bot assistant. Respond briefly and helpfully.".to_string(),
            enabled: false,
        }
    }
}
