//! Модуль для проверок серверов и конфигураций
//! 
//! Этот модуль содержит функции для проверки доступности серверов,
// валидации конфигураций и диагностики проблем.

pub mod server_check;
pub mod config_check;
pub mod ai_check;
pub mod discord_check;
pub mod bot_state_check;

pub use server_check::{check_server, check_minecraft_server, ServerCheckResult};
pub use config_check::{validate_bot_config, ConfigCheckResult};
pub use ai_check::{check_ai_config, check_ai_availability, AICheckResult};
pub use discord_check::{check_discord_config, check_discord_api, DiscordCheckResult};
pub use bot_state_check::{check_bot_state, BotStateCheckResult};
