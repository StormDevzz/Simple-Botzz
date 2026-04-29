use anyhow::Result;

/// Результат проверки Discord конфигурации
#[derive(Debug, Clone)]
pub struct DiscordCheckResult {
    pub valid: bool,
    pub client_id_valid: bool,
    pub public_key_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl Default for DiscordCheckResult {
    fn default() -> Self {
        Self {
            valid: false,
            client_id_valid: false,
            public_key_valid: false,
            errors: vec![],
            warnings: vec![],
        }
    }
}

/// Проверяет валидность Discord конфигурации
pub fn check_discord_config(client_id: &str, public_key: &str) -> Result<DiscordCheckResult> {
    let mut result = DiscordCheckResult::default();

    // Проверяем client_id
    if client_id.is_empty() {
        result.errors.push("Discord Client ID is empty".to_string());
    } else if client_id.parse::<u64>().is_err() {
        result.errors.push("Discord Client ID must be a valid number".to_string());
    } else if client_id.len() < 17 {
        result.warnings.push("Discord Client ID seems too short".to_string());
    } else {
        result.client_id_valid = true;
    }

    // Проверяем public_key
    if public_key.is_empty() {
        result.errors.push("Discord Public Key is empty".to_string());
    } else if public_key.len() != 64 {
        result.warnings.push("Discord Public Key should be 64 characters (hex)".to_string());
    } else if !is_valid_hex(public_key) {
        result.errors.push("Discord Public Key must be valid hexadecimal".to_string());
    } else {
        result.public_key_valid = true;
    }

    // Определяем общую валидность
    result.valid = result.client_id_valid && result.public_key_valid;

    Ok(result)
}

/// Проверяет, является ли строка валидным hex
fn is_valid_hex(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_hexdigit())
}

/// Проверяет доступность Discord API (симуляция)
pub async fn check_discord_api(client_id: &str) -> Result<bool> {
    // В реальности здесь был бы запрос к Discord API
    // Для примера просто проверяем формат
    Ok(!client_id.is_empty() && client_id.parse::<u64>().is_ok())
}
