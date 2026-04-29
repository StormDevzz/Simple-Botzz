use std::sync::{Arc, Mutex};

/// Просмотрщик консоли для отображения вывода бота
pub struct ConsoleViewer {
    output_lines: Arc<Mutex<Vec<String>>>,
    max_lines: usize,
}

impl ConsoleViewer {
    /// Создает новый просмотрщик консоли
    pub fn new(max_lines: usize) -> Self {
        Self {
            output_lines: Arc::new(Mutex::new(vec![])),
            max_lines,
        }
    }

    /// Добавляет строку в вывод
    pub fn add_line(&self, line: String) {
        let mut lines = self.output_lines.lock().unwrap();
        lines.push(line);
        if lines.len() > self.max_lines {
            lines.remove(0);
        }
    }

    /// Возвращает все строки вывода
    pub fn get_lines(&self) -> Vec<String> {
        self.output_lines.lock().unwrap().clone()
    }

    /// Очищает вывод
    pub fn clear(&self) {
        self.output_lines.lock().unwrap().clear();
    }

    /// Возвращает ссылку на Arc с выводом
    pub fn get_output_arc(&self) -> Arc<Mutex<Vec<String>>> {
        self.output_lines.clone()
    }
}
