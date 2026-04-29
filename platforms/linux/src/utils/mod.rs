//! Утилиты и вспомогательные функции
//! 
//! Этот модуль содержит различные вспомогательные функции для работы
//! с файлами, конфигурациями и другими общими задачами.

pub mod file_utils;
pub mod config_utils;
pub mod validation;

pub use file_utils::{ensure_dir, read_file, write_file};
pub use config_utils::{load_toml, save_toml};
pub use validation::{validate_ip, validate_port, validate_username};
