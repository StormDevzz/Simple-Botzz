use serde::{Deserialize, Serialize};

/// Тип сообщения для моста Rust-Node.js
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    /// Запрос статуса бота
    BotStatus,
    /// Команда запуска бота
    StartBot,
    /// Команда остановки бота
    StopBot,
    /// Команда перезапуска бота
    RestartBot,
    /// Получение логов бота
    GetLogs,
    /// Обновление конфигурации
    UpdateConfig,
    /// Пинг
    Ping,
    /// Понг
    Pong,
    /// Ошибка
    Error,
    /// Данные
    Data,
}

/// Сообщение для моста Rust-Node.js
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeMessage {
    /// Тип сообщения
    pub message_type: MessageType,
    /// ID бота (если применимо)
    pub bot_id: Option<String>,
    /// Данные сообщения
    pub data: serde_json::Value,
    /// Временная метка
    pub timestamp: i64,
}

impl BridgeMessage {
    pub fn new(message_type: MessageType, bot_id: Option<String>, data: serde_json::Value) -> Self {
        Self {
            message_type,
            bot_id,
            data,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
