use anyhow::Result;

/// Проверяет валидность API ключа
pub fn validate_api_key(api_key: &str, provider: &str) -> Result<bool> {
    if api_key.is_empty() {
        return Ok(false);
    }

    match provider {
        "openai" => validate_openai_key(api_key),
        "anthropic" => validate_anthropic_key(api_key),
        _ => Ok(true), // Для локальных моделей ключ не требуется
    }
}

/// Проверяет ключ OpenAI
fn validate_openai_key(api_key: &str) -> Result<bool> {
    // OpenAI ключи начинаются с "sk-"
    if !api_key.starts_with("sk-") {
        return Ok(false);
    }

    // Ключ должен быть длиной примерно 51 символ
    if api_key.len() < 40 {
        return Ok(false);
    }

    // TODO: Реализовать реальную проверку через API
    Ok(true)
}

/// Проверяет ключ Anthropic
fn validate_anthropic_key(api_key: &str) -> Result<bool> {
    // Anthropic ключи начинаются с "sk-ant-"
    if !api_key.starts_with("sk-ant-") {
        return Ok(false);
    }

    // TODO: Реализовать реальную проверку через API
    Ok(true)
}

/// Проверяет валидность названия модели
pub fn validate_model(model: &str, provider: &str) -> Result<bool> {
    if model.is_empty() {
        return Ok(false);
    }

    match provider {
        "openai" => validate_openai_model(model),
        "anthropic" => validate_anthropic_model(model),
        "local" => Ok(true), // Локальные модели могут быть любыми
        _ => Ok(false),
    }
}

/// Проверяет модель OpenAI
fn validate_openai_model(model: &str) -> Result<bool> {
    let valid_models = vec![
        "gpt-4",
        "gpt-4-turbo",
        "gpt-4-turbo-preview",
        "gpt-3.5-turbo",
        "gpt-3.5-turbo-16k",
    ];

    Ok(valid_models.contains(&model) || model.starts_with("gpt-"))
}

/// Проверяет модель Anthropic
fn validate_anthropic_model(model: &str) -> Result<bool> {
    let valid_models = vec![
        "claude-3-opus-20240229",
        "claude-3-sonnet-20240229",
        "claude-3-haiku-20240307",
        "claude-2.1",
        "claude-2",
    ];

    Ok(valid_models.contains(&model) || model.starts_with("claude-"))
}

/// Проверяет доступность модели через API
pub async fn check_model_availability(api_key: &str, model: &str, provider: &str) -> Result<bool> {
    // TODO: Реализовать реальную проверку через API
    match provider {
        "openai" => check_openai_model(api_key, model).await,
        "anthropic" => check_anthropic_model(api_key, model).await,
        _ => Ok(true),
    }
}

async fn check_openai_model(api_key: &str, model: &str) -> Result<bool> {
    // TODO: Реализовать запрос к OpenAI API для проверки модели
    Ok(!api_key.is_empty() && !model.is_empty())
}

async fn check_anthropic_model(api_key: &str, model: &str) -> Result<bool> {
    // TODO: Реализовать запрос к Anthropic API для проверки модели
    Ok(!api_key.is_empty() && !model.is_empty())
}
