use crate::bots::bot_config::BotConfig;
use crate::utils::validation::{validate_ip, validate_port, validate_username};

/// Результат проверки конфигурации
#[derive(Debug, Clone)]
pub struct ConfigCheckResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Проверяет валидность конфигурации бота
pub fn validate_bot_config(bot: &BotConfig) -> ConfigCheckResult {
    let mut errors = vec![];
    let mut warnings = vec![];
    
    // Проверка имени
    if bot.name.is_empty() {
        errors.push("Имя бота не может быть пустым".to_string());
    }
    
    // Проверка сервера
    if bot.server.is_empty() {
        errors.push("Сервер не указан".to_string());
    } else if !validate_ip(&bot.server) && !bot.server.contains('.') {
        warnings.push("Сервер может быть недействительным IP или доменом".to_string());
    }
    
    // Проверка порта
    if !validate_port(bot.port) {
        errors.push(format!("Неверный порт: {}", bot.port));
    }
    
    // Проверка имени пользователя
    if !validate_username(&bot.username) {
        errors.push(format!("Неверное имя пользователя: {}", bot.username));
    }
    
    // Проверка пароля для онлайн режима
    if bot.account_type == crate::bots::bot_config::AccountType::Online && bot.password.is_empty() {
        warnings.push("Пароль не указан для онлайн режима".to_string());
    }
    
    // Проверка скрипта
    if !bot.use_external_script && bot.script_content.is_empty() {
        errors.push("Скрипт бота пуст".to_string());
    }
    
    if bot.use_external_script {
        if let Some(ref path) = bot.script_path {
            if !std::path::Path::new(path).exists() {
                errors.push(format!("Скрипт не найден: {}", path));
            }
        } else {
            errors.push("Внешний скрипт выбран, но путь не указан".to_string());
        }
    }
    
    ConfigCheckResult {
        valid: errors.is_empty(),
        errors,
        warnings,
    }
}
