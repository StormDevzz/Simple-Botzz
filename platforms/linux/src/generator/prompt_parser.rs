use crate::bots::bot_config::AccountType;
use anyhow::Result;

/// Парсит промпт и извлекает параметры для бота
pub fn parse_prompt(prompt: &str) -> Result<BotPromptParams> {
    let mut params = BotPromptParams::default();
    
    // Анализ промпта на ключевые слова
    let lower_prompt = prompt.to_lowercase();
    
    // Определение типа аккаунта
    if lower_prompt.contains("онлайн") || lower_prompt.contains("лицензия") || lower_prompt.contains("microsoft") {
        params.account_type = AccountType::Online;
    } else if lower_prompt.contains("оффлайн") || lower_prompt.contains("без лицензии") {
        params.account_type = AccountType::Offline;
    }
    
    // Извлечение IP/сервера
    if let Some(ip) = extract_ip(&lower_prompt) {
        params.server = ip;
    }
    
    // Извлечение порта
    if let Some(port) = extract_port(&lower_prompt) {
        params.port = port;
    }
    
    // Извлечение имени пользователя
    if let Some(username) = extract_username(prompt) {
        params.username = username;
    }
    
    // Извлечение имени бота
    if let Some(name) = extract_bot_name(prompt) {
        params.name = name;
    }
    
    // Проверка на авто-логин
    if lower_prompt.contains("авто логин") || lower_prompt.contains("автологин") {
        params.auto_login = true;
    }
    
    // Извлечение авто-сообщений
    params.auto_messages = extract_auto_messages(prompt);
    
    Ok(params)
}

/// Параметры бота, извлеченные из промпта
#[derive(Debug, Default)]
pub struct BotPromptParams {
    pub name: String,
    pub server: String,
    pub port: u16,
    pub account_type: AccountType,
    pub username: String,
    pub password: String,
    pub auto_login: bool,
    pub auto_messages: Vec<String>,
}

/// Извлекает IP адрес из текста
fn extract_ip(text: &str) -> Option<String> {
    // Простое извлечение IP по шаблону xxx.xxx.xxx.xxx
    let ip_pattern = regex::Regex::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b").ok()?;
    if let Some(caps) = ip_pattern.find(text) {
        return Some(caps.as_str().to_string());
    }
    
    // Поиск доменного имени
    let domain_pattern = regex::Regex::new(r"\b[a-zA-Z0-9-]+\.[a-zA-Z]{2,}\b").ok()?;
    if let Some(caps) = domain_pattern.find(text) {
        return Some(caps.as_str().to_string());
    }
    
    None
}

/// Извлекает порт из текста
fn extract_port(text: &str) -> Option<u16> {
    let port_pattern = regex::Regex::new(r"порт[:\s]*(\d{1,5})").ok()?;
    if let Some(caps) = port_pattern.captures(text) {
        if let Some(port_str) = caps.get(1) {
            return port_str.as_str().parse().ok();
        }
    }
    
    // Поиск порта после двоеточия
    let colon_pattern = regex::Regex::new(r":(\d{1,5})\b").ok()?;
    if let Some(caps) = colon_pattern.captures(text) {
        if let Some(port_str) = caps.get(1) {
            let port: u16 = port_str.as_str().parse().ok()?;
            if port > 1024 && port <= 65535 {
                return Some(port);
            }
        }
    }
    
    None
}

/// Извлекает имя пользователя из текста
fn extract_username(text: &str) -> Option<String> {
    // Поиск после "ник" или "имя"
    let patterns = vec![
        r"ник[:\s]+([a-zA-Z0-9_]+)",
        r"имя[:\s]+([a-zA-Z0-9_]+)",
        r"username[:\s]+([a-zA-Z0-9_]+)",
    ];
    
    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if let Some(caps) = re.captures(text) {
                if let Some(username) = caps.get(1) {
                    return Some(username.as_str().to_string());
                }
            }
        }
    }
    
    None
}

/// Извлекает имя бота из текста
fn extract_bot_name(text: &str) -> Option<String> {
    let patterns = vec![
        r"бот[:\s]+([a-zA-Z0-9_]+)",
        r"назвать[:\s]+([a-zA-Z0-9_]+)",
    ];
    
    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if let Some(caps) = re.captures(text) {
                if let Some(name) = caps.get(1) {
                    return Some(name.as_str().to_string());
                }
            }
        }
    }
    
    None
}

/// Извлекает авто-сообщения из текста
fn extract_auto_messages(text: &str) -> Vec<String> {
    let mut messages = vec![];
    
    // Поиск сообщений в кавычках
    let quote_pattern = regex::Regex::new(r#""([^"]+)""#).ok();
    if let Some(re) = quote_pattern {
        for caps in re.captures_iter(text) {
            if let Some(msg) = caps.get(1) {
                messages.push(msg.as_str().to_string());
            }
        }
    }
    
    messages
}
