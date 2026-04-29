use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

/// Логгер для записи вывода бота в файл
pub struct Logger {
    log_file: PathBuf,
}

impl Logger {
    /// Создает новый логгер
    pub fn new(log_dir: &str, bot_id: &str) -> Self {
        let log_file = PathBuf::from(format!("{}/{}.log", log_dir, bot_id));
        
        // Создаем директорию если не существует
        if let Some(parent) = log_file.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        
        Self { log_file }
    }

    /// Записывает сообщение в лог
    pub fn log(&self, message: &str) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)
        {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            let _ = writeln!(file, "[{}] {}", timestamp, message);
        }
    }

    /// Очищает лог-файл
    pub fn clear(&self) {
        let _ = std::fs::write(&self.log_file, "");
    }
}
