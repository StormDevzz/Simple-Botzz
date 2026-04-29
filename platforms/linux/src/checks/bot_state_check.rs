use anyhow::Result;
use std::time::{Duration, Instant};

/// Статус бота
#[derive(Debug, Clone, PartialEq)]
pub enum BotConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Authenticating,
    Authenticated,
    Error(String),
}

/// Результат проверки состояния бота
#[derive(Debug, Clone)]
pub struct BotStateCheckResult {
    pub state: BotConnectionState,
    pub connected: bool,
    pub authenticated: bool,
    pub ping_ms: Option<u64>,
    pub last_activity: Option<Instant>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl Default for BotStateCheckResult {
    fn default() -> Self {
        Self {
            state: BotConnectionState::Disconnected,
            connected: false,
            authenticated: false,
            ping_ms: None,
            last_activity: None,
            errors: vec![],
            warnings: vec![],
        }
    }
}

/// Проверяет состояние бота
pub fn check_bot_state(
    is_running: bool,
    last_ping: Option<Instant>,
    error_count: u32,
) -> Result<BotStateCheckResult> {
    let mut result = BotStateCheckResult::default();

    if !is_running {
        result.state = BotConnectionState::Disconnected;
        result.errors.push("Bot is not running".to_string());
        return Ok(result);
    }

    // Проверяем время последней активности
    if let Some(last) = last_ping {
        let elapsed = last.elapsed();
        result.last_activity = Some(last);
        
        if elapsed > Duration::from_secs(30) {
            result.warnings.push("No activity for 30+ seconds".to_string());
            result.state = BotConnectionState::Error("Timeout".to_string());
        } else if elapsed > Duration::from_secs(10) {
            result.warnings.push("No activity for 10+ seconds".to_string());
            result.state = BotConnectionState::Connected;
        } else {
            result.state = BotConnectionState::Connected;
            result.connected = true;
        }
    } else {
        result.state = BotConnectionState::Connecting;
        result.warnings.push("No ping data available".to_string());
    }

    // Проверяем количество ошибок
    if error_count > 10 {
        result.errors.push(format!("Too many errors: {}", error_count));
        result.state = BotConnectionState::Error("Error limit exceeded".to_string());
    } else if error_count > 5 {
        result.warnings.push(format!("Warning: {} errors occurred", error_count));
    }

    Ok(result)
}

/// Проверяет аутентификацию бота
pub fn check_authentication(
    username: &str,
    password: &str,
    account_type: &str,
) -> Result<AuthCheckResult> {
    let mut result = AuthCheckResult::default();

    if username.is_empty() {
        result.errors.push("Username is empty".to_string());
    } else if username.len() < 3 {
        result.warnings.push("Username is very short".to_string());
    } else {
        result.username_valid = true;
    }

    if account_type == "online" && password.is_empty() {
        result.errors.push("Password required for online accounts".to_string());
    } else if account_type == "online" && password.len() < 6 {
        result.warnings.push("Password seems weak".to_string());
    }

    result.valid = result.username_valid && result.errors.is_empty();

    Ok(result)
}

/// Результат проверки аутентификации
#[derive(Debug, Clone, Default)]
pub struct AuthCheckResult {
    pub valid: bool,
    pub username_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Проверяет состояние скрипта бота
pub fn check_script_state(script_content: &str, use_external: bool, script_path: Option<&str>) -> Result<ScriptCheckResult> {
    let mut result = ScriptCheckResult::default();

    if use_external {
        if let Some(path) = script_path {
            if path.is_empty() {
                result.errors.push("External script path is empty".to_string());
            } else {
                result.script_exists = std::path::Path::new(path).exists();
                if !result.script_exists {
                    result.errors.push(format!("Script file not found: {}", path));
                }
            }
        } else {
            result.errors.push("External script enabled but no path provided".to_string());
        }
    } else {
        if script_content.is_empty() {
            result.warnings.push("Script content is empty".to_string());
        } else {
            result.syntax_valid = !script_content.contains("syntax error");
            if !result.syntax_valid {
                result.errors.push("Script may contain syntax errors".to_string());
            }
        }
    }

    result.valid = result.errors.is_empty();

    Ok(result)
}

/// Результат проверки скрипта
#[derive(Debug, Clone, Default)]
pub struct ScriptCheckResult {
    pub valid: bool,
    pub script_exists: bool,
    pub syntax_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}
