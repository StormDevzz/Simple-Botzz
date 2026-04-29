use super::ai_config::AIConfig;
use anyhow::Result;

/// Клиент для ИИ API
pub struct AIClient {
    config: AIConfig,
}

impl AIClient {
    pub fn new(config: AIConfig) -> Self {
        Self { config }
    }

    /// Отправляет сообщение в ИИ и получает ответ
    pub async fn send_message(&self, message: &str) -> Result<String> {
        if !self.config.enabled {
            return Ok("AI chat is disabled".to_string());
        }

        if self.config.api_key.is_empty() {
            return Ok("API key is not set".to_string());
        }

        // TODO: Реализовать реальный запрос к API
        // Для примера возвращаем заглушку
        match self.config.provider.as_str() {
            "openai" => self.send_openai_request(message).await,
            "anthropic" => self.send_anthropic_request(message).await,
            "local" => self.send_local_request(message).await,
            _ => Ok("Unknown provider".to_string()),
        }
    }

    async fn send_openai_request(&self, message: &str) -> Result<String> {
        // TODO: Реализовать запрос к OpenAI API
        // Используем reqwest для HTTP запросов
        Ok(format!("[OpenAI Response to: {}]", message))
    }

    async fn send_anthropic_request(&self, message: &str) -> Result<String> {
        // TODO: Реализовать запрос к Anthropic API
        Ok(format!("[Anthropic Response to: {}]", message))
    }

    async fn send_local_request(&self, message: &str) -> Result<String> {
        // TODO: Реализовать запрос к локальной модели (Ollama и т.д.)
        Ok(format!("[Local AI Response to: {}]", message))
    }

    /// Получает конфигурацию
    pub fn get_config(&self) -> &AIConfig {
        &self.config
    }

    /// Устанавливает конфигурацию
    pub fn set_config(&mut self, config: AIConfig) {
        self.config = config;
    }
}
