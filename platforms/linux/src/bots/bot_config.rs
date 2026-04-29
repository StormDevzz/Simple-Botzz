use serde::{Deserialize, Serialize};

/// Тип аккаунта бота
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum AccountType {
    /// Оффлайн режим (без авторизации)
    Offline,
    /// Онлайн режим с лицензией Microsoft
    Online,
}

impl Default for AccountType {
    fn default() -> Self {
        AccountType::Offline
    }
}

/// Конфигурация отдельного бота
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotConfig {
    /// Уникальный идентификатор бота
    pub id: String,
    /// Отображаемое имя бота
    pub name: String,
    /// Путь к внешнему скрипту (если используется)
    pub script_path: Option<String>,
    /// Содержимое встроенного скрипта
    pub script_content: String,
    /// IP адрес или домен сервера
    pub server: String,
    /// Порт сервера
    pub port: u16,
    /// Тип аккаунта
    pub account_type: AccountType,
    /// Имя пользователя (никнейм)
    pub username: String,
    /// Пароль (для онлайн режима)
    pub password: String,
    /// Включен ли бот
    pub enabled: bool,
    /// Текущий статус бота
    #[serde(default)]
    pub status: String,
    /// Использовать внешний скрипт
    pub use_external_script: bool,
    /// Использовать автоматически сгенерированный скрипт
    pub use_generated_script: bool,
    /// Авто-логин при подключении
    pub auto_login: bool,
    /// Пароль для авто-логина
    pub auto_login_password: String,
    /// Авто-сообщения при входе
    pub auto_messages: Vec<String>,
}

/// Глобальная конфигурация приложения
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Сервер по умолчанию
    pub server: String,
    /// Порт по умолчанию
    pub port: u16,
    /// Список ботов
    pub bots: Vec<BotConfig>,
}
