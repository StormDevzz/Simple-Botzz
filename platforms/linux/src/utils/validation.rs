/// Проверяет валидность IP адреса
pub fn validate_ip(ip: &str) -> bool {
    // Простая проверка IP адреса
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return false;
    }
    
    for part in parts {
        if let Ok(num) = part.parse::<u8>() {
            // Валидный октет
        } else {
            // Может быть доменное имя
            return ip.contains('.') && !ip.contains(' ');
        }
    }
    
    true
}

/// Проверяет валидность порта
pub fn validate_port(port: u16) -> bool {
    port > 0 && port <= 65535
}

/// Проверяет валидность имени пользователя
pub fn validate_username(username: &str) -> bool {
    !username.is_empty() && username.len() <= 16 && username.chars().all(|c| c.is_alphanumeric() || c == '_')
}
