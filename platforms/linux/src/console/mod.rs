//! Модуль для работы с консолью и выводом
//! 
//! Этот модуль содержит функции для отображения консоли бота,
//! обработки вывода и управления логами.

pub mod viewer;
pub mod logger;

pub use viewer::ConsoleViewer;
pub use logger::Logger;
