//! Модуль для генерации и управления скриптами ботов
//! 
//! Этот модуль содержит функции для генерации скриптов на основе
//! параметров бота и шаблонов.

pub mod generator;
pub mod templates;

pub use generator::{generate_script_from_params, get_default_script};
pub use templates::{get_minecraft_bot_template, get_chat_bot_template};
