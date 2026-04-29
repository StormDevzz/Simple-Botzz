use crate::ai::ai_checks::{validate_api_key, validate_model, check_model_availability};
use anyhow::Result;

/// Результат проверки ИИ конфигурации
#[derive(Debug, Clone)]
pub struct AICheckResult {
    pub valid: bool,
    pub api_key_valid: bool,
    pub model_valid: bool,
    pub model_available: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl Default for AICheckResult {
    fn default() -> Self {
        Self {
            valid: false,
            api_key_valid: false,
            model_valid: false,
            model_available: false,
            errors: vec![],
            warnings: vec![],
        }
    }
}

/// Проверяет конфигурацию ИИ
pub fn check_ai_config(api_key: &str, model: &str, provider: &str) -> Result<AICheckResult> {
    let mut result = AICheckResult::default();

    // Проверяем API ключ
    match validate_api_key(api_key, provider) {
        Ok(valid) => {
            result.api_key_valid = valid;
            if !valid {
                result.errors.push("Invalid API key format".to_string());
            }
        }
        Err(e) => {
            result.errors.push(format!("API key validation error: {}", e));
        }
    }

    // Проверяем модель
    match validate_model(model, provider) {
        Ok(valid) => {
            result.model_valid = valid;
            if !valid {
                result.errors.push("Invalid model name".to_string());
            }
        }
        Err(e) => {
            result.errors.push(format!("Model validation error: {}", e));
        }
    }

    // Проверяем, если ключ или модель пустые
    if api_key.is_empty() {
        result.warnings.push("API key is empty".to_string());
    }

    if model.is_empty() {
        result.warnings.push("Model is empty".to_string());
    }

    // Определяем общую валидность
    result.valid = result.api_key_valid && result.model_valid;

    Ok(result)
}

/// Асинхронно проверяет доступность модели
pub async fn check_ai_availability(api_key: &str, model: &str, provider: &str) -> Result<bool> {
    check_model_availability(api_key, model, provider).await
}
