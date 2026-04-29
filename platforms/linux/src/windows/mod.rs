//! Модуль для управления окнами приложения
//! 
//! Этот модуль содержит структуры и функции для создания и управления
//! различными окнами приложения (редактор, консоль, генератор и т.д.)

pub mod editor_window;
pub mod console_window;
pub mod generator_window;
pub mod logs_window;

pub use editor_window::EditorWindow;
pub use console_window::ConsoleWindow;
pub use generator_window::GeneratorWindow;
pub use logs_window::LogsWindow;
