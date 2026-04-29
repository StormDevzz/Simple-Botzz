use std::fs;
use std::path::Path;
use anyhow::Result;

/// Убедится, что директория существует
pub fn ensure_dir(path: &str) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

/// Читает содержимое файла
pub fn read_file(path: &str) -> Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

/// Записывает содержимое в файл
pub fn write_file(path: &str, content: &str) -> Result<()> {
    fs::write(path, content)?;
    Ok(())
}

/// Проверяет существование файла
pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// Удаляет файл если существует
pub fn remove_file(path: &str) -> Result<()> {
    if Path::new(path).exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}
