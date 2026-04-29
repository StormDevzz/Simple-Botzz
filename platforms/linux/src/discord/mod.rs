//! Модуль для Discord Rich Presence интеграции
//! 
//! Отображает статус бота в Discord

pub mod rich_presence;
pub mod discord_config_loader;

pub use rich_presence::{DiscordRichPresence, DiscordConfig};
pub use discord_config_loader::{load_discord_config, save_discord_config};
