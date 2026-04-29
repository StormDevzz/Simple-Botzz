use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Загружает конфигурацию из TOML файла
pub fn load_toml<T: DeserializeOwned>(path: &str) -> Result<T> {
    let content = crate::utils::file_utils::read_file(path)?;
    let config: T = toml::from_str(&content)?;
    Ok(config)
}

/// Сохраняет конфигурацию в TOML файл
pub fn save_toml<T: Serialize>(config: &T, path: &str) -> Result<()> {
    let content = toml::to_string_pretty(config)?;
    crate::utils::file_utils::write_file(path, &content)?;
    Ok(())
}
