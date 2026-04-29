//! Модуль для генерации ботов по промпту
//! 
//! Этот модуль содержит функции для генерации конфигураций ботов
//! на основе текстовых описаний (промптов).

pub mod prompt_parser;
pub mod bot_builder;

pub use prompt_parser::parse_prompt;
pub use bot_builder::build_bot_from_prompt;
