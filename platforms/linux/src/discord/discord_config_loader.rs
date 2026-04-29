use anyhow::Result;
use std::env;

/// Загружает Discord конфигурацию из переменных окружения или использует значения по умолчанию
pub fn load_discord_config() -> (String, String, bool) {
    // Загружаем .env файл если он существует
    let _ = dotenvy::dotenv();
    
    // Получаем значения из переменных окружения или используем пустые строки
    let client_id = env::var("DISCORD_CLIENT_ID").unwrap_or_default();
    let public_key = env::var("DISCORD_PUBLIC_KEY").unwrap_or_default();
    let enabled = env::var("DISCORD_ENABLED").map(|v| v == "true").unwrap_or(false);
    
    // Если переменные окружения не установлены, пытаемся загрузить из файла
    if client_id.is_empty() || public_key.is_empty() {
        if let Ok(config) = load_from_file() {
            return config;
        }
    }
    
    (client_id, public_key, enabled)
}

/// Загружает конфигурацию из файла discord_config.toml
fn load_from_file() -> Result<(String, String, bool)> {
    let config_str = std::fs::read_to_string("discord_config.toml")?;
    let config: toml::Value = toml::from_str(&config_str)?;
    
    let client_id = config.get("client_id")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    
    let public_key = config.get("public_key")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    
    let enabled = config.get("enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    
    Ok((client_id, public_key, enabled))
}

/// Сохраняет конфигурацию в файл
pub fn save_discord_config(client_id: &str, public_key: &str, enabled: bool) -> Result<()> {
    let config = format!(
        r#"# Discord Rich Presence Configuration
client_id = "{}")
public_key = "{}")
enabled = {}
"#,
        client_id, public_key, enabled
    );
    
    std::fs::write("discord_config.toml", config)?;
    Ok(())
}
