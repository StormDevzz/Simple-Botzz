use crate::bots::bot_config::{BotConfig, AccountType};
use super::prompt_parser::BotPromptParams;
use crate::scripts::get_default_script;

/// Строит конфигурацию бота из параметров промпта
pub fn build_bot_from_prompt(params: BotPromptParams, existing_bots_count: usize) -> BotConfig {
    let name = if params.name.is_empty() {
        format!("GeneratedBot{}", existing_bots_count + 1)
    } else {
        params.name
    };
    
    let server = if params.server.is_empty() {
        "localhost".to_string()
    } else {
        params.server
    };
    
    let port = if params.port == 0 {
        25565
    } else {
        params.port
    };
    
    let username = if params.username.is_empty() {
        "Player".to_string()
    } else {
        params.username
    };
    
    BotConfig {
        id: format!("bot{}", existing_bots_count + 1),
        name,
        script_path: None,
        script_content: get_default_script(),
        server,
        port,
        account_type: params.account_type,
        username,
        password: params.password,
        enabled: true,
        status: "Остановлен".to_string(),
        use_external_script: false,
        use_generated_script: true,
        auto_login: params.auto_login,
        auto_login_password: String::new(),
        auto_messages: params.auto_messages,
    }
}
