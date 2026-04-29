//! Модуль для ИИ чат-бота
//! 
//! Интеграция с API OpenAI и другими моделями

pub mod ai_config;
pub mod ai_client;
pub mod ai_checks;

pub use ai_config::AIConfig;
pub use ai_client::AIClient;
pub use ai_checks::{validate_api_key, validate_model};
