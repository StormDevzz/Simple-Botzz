//! Модуль для управления ботами
//! 
//! Этот модуль содержит структуры и функции для управления ботами,
//! включая их конфигурацию, запуск и остановку.

pub mod bot_manager;
pub mod bot_config;

pub use bot_manager::{BotProcess};
pub use bot_config::{BotConfig, Config, AccountType};
