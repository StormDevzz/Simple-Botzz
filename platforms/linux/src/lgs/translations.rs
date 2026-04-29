use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Russian,
    English,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Russian => "ru",
            Language::English => "en",
        }
    }
}

pub struct Translator {
    current_lang: Language,
    translations: HashMap<Language, HashMap<String, String>>,
}

impl Translator {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        translations.insert(Language::Russian, Self::russian_translations());
        translations.insert(Language::English, Self::english_translations());
        
        Self {
            current_lang: Language::Russian,
            translations,
        }
    }

    pub fn set_language(&mut self, lang: Language) {
        self.current_lang = lang;
    }

    pub fn get_language(&self) -> Language {
        self.current_lang
    }

    pub fn t(&self, key: &str) -> String {
        self.translations
            .get(&self.current_lang)
            .and_then(|map| map.get(key))
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    fn russian_translations() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("app_title".to_string(), "Js Simple Botzz - Менеджер Ботов".to_string());
        map.insert("bot_list".to_string(), "Список ботов:".to_string());
        map.insert("add_bot".to_string(), "Добавить бота".to_string());
        map.insert("generator".to_string(), "Генератор ботов".to_string());
        map.insert("stop_all".to_string(), "Остановить всех".to_string());
        map.insert("settings".to_string(), "Настройки бота".to_string());
        map.insert("id".to_string(), "ID:".to_string());
        map.insert("name".to_string(), "Имя:".to_string());
        map.insert("server".to_string(), "Сервер:".to_string());
        map.insert("port".to_string(), "Порт:".to_string());
        map.insert("account_type".to_string(), "Тип аккаунта:".to_string());
        map.insert("offline".to_string(), "Оффлайн".to_string());
        map.insert("online".to_string(), "Онлайн (Microsoft)".to_string());
        map.insert("username".to_string(), "Имя пользователя:".to_string());
        map.insert("password".to_string(), "Пароль:".to_string());
        map.insert("auto_login".to_string(), "Авто-логин".to_string());
        map.insert("auto_messages".to_string(), "Авто-сообщения при входе:".to_string());
        map.insert("script_type".to_string(), "Тип скрипта:".to_string());
        map.insert("auto_generation".to_string(), "Автоматическая генерация".to_string());
        map.insert("built_in_editor".to_string(), "Встроенный редактор".to_string());
        map.insert("external_file".to_string(), "Внешний файл".to_string());
        map.insert("script_editor".to_string(), "Редактор скрипта".to_string());
        map.insert("open_editor".to_string(), "Открыть редактор".to_string());
        map.insert("regenerate".to_string(), "Перегенерировать".to_string());
        map.insert("enabled".to_string(), "Включен".to_string());
        map.insert("status".to_string(), "Статус".to_string());
        map.insert("start".to_string(), "Запустить".to_string());
        map.insert("stop".to_string(), "Остановить".to_string());
        map.insert("save".to_string(), "Сохранить".to_string());
        map.insert("delete".to_string(), "Удалить".to_string());
        map.insert("console".to_string(), "Консоль".to_string());
        map.insert("update".to_string(), "Обновить".to_string());
        map.insert("clear".to_string(), "Очистить".to_string());
        map.insert("close".to_string(), "Закрыть".to_string());
        map.insert("apply".to_string(), "Применить".to_string());
        map.insert("cancel".to_string(), "Отменить".to_string());
        map.insert("delete_script".to_string(), "Удалить скрипт".to_string());
        map.insert("total_bots".to_string(), "Всего ботов".to_string());
        map.insert("default_server".to_string(), "Сервер по умолчанию".to_string());
        map.insert("default_port".to_string(), "Порт по умолчанию".to_string());
        map.insert("add_message".to_string(), "Добавить сообщение".to_string());
        map.insert("automation".to_string(), "Автоматизация".to_string());
        map.insert("prompt_placeholder".to_string(), "Опишите бота (пример: 'Создать бота с ником Steve для сервера 192.168.1.1:25565, оффлайн режим, авто-логин')".to_string());
        map.insert("generate".to_string(), "Сгенерировать".to_string());
        map.insert("server_check".to_string(), "Проверка сервера".to_string());
        map.insert("config_check".to_string(), "Проверка конфигурации".to_string());
        map.insert("language".to_string(), "Язык".to_string());
        map.insert("connection_params".to_string(), "Параметры подключения:".to_string());
        map.insert("script_auto_gen".to_string(), "Скрипт генерируется автоматически на основе параметров".to_string());
        map.insert("script_path".to_string(), "Путь к скрипту:".to_string());
        map.insert("not_specified".to_string(), "Не указан".to_string());
        map.insert("select_bot".to_string(), "Выберите бота из списка слева".to_string());
        map.insert("edit_script".to_string(), "Редактируйте скрипт бота:".to_string());
        map.insert("console_output".to_string(), "Вывод консоли бота:".to_string());
        map.insert("server_check_result".to_string(), "Результат проверки сервера:".to_string());
        map.insert("server_available".to_string(), "Сервер доступен (пинг: {}мс)".to_string());
        map.insert("server_unavailable".to_string(), "Сервер недоступен: {}".to_string());
        map.insert("config_check_result".to_string(), "Результат проверки конфигурации:".to_string());
        map.insert("config_valid".to_string(), "Конфигурация валидна".to_string());
        map.insert("select_file".to_string(), "Выбрать файл".to_string());
        map.insert("enabled_status".to_string(), "Включен: {}".to_string());
        map.insert("status_label".to_string(), "Статус: {}".to_string());
        map.insert("logs".to_string(), "Логи".to_string());
        map.insert("bot_logs".to_string(), "Логи бота".to_string());
        map.insert("refresh".to_string(), "Обновить".to_string());
        map.insert("export".to_string(), "Экспорт".to_string());
        map.insert("auto_password".to_string(), "Пароль для авто-логина:".to_string());
        map.insert("ai_settings".to_string(), "Настройки ИИ".to_string());
        map.insert("discord_settings".to_string(), "Настройки Discord".to_string());
        map.insert("packet_settings".to_string(), "Настройки пакетов".to_string());
        map.insert("api_key".to_string(), "API ключ:".to_string());
        map.insert("ai_model".to_string(), "Модель:".to_string());
        map.insert("ai_provider".to_string(), "Провайдер:".to_string());
        map.insert("max_tokens".to_string(), "Макс. токены:".to_string());
        map.insert("temperature".to_string(), "Температура:".to_string());
        map.insert("system_prompt".to_string(), "Системный промпт:".to_string());
        map.insert("enable_ai".to_string(), "Включить ИИ".to_string());
        map.insert("discord_enabled".to_string(), "Discord Rich Presence:".to_string());
        map.insert("rate_limit".to_string(), "Rate limiting:".to_string());
        map.insert("packet_filter".to_string(), "Фильтр пакетов:".to_string());
        map.insert("min_interval".to_string(), "Мин. интервал (мс):".to_string());
        map.insert("check_ai".to_string(), "Проверить ИИ".to_string());
        map.insert("ai_check_result".to_string(), "Результат проверки ИИ:".to_string());
        map.insert("theme_settings".to_string(), "Настройки темы".to_string());
        map.insert("background_color".to_string(), "Цвет фона:".to_string());
        map.insert("panel_color".to_string(), "Цвет панели:".to_string());
        map.insert("text_color".to_string(), "Цвет текста:".to_string());
        map.insert("accent_color".to_string(), "Акцентный цвет:".to_string());
        map.insert("button_color".to_string(), "Цвет кнопки:".to_string());
        map.insert("border_color".to_string(), "Цвет границы:".to_string());
        map.insert("auto_register".to_string(), "Авто-регистрация".to_string());
        map.insert("auto_register_password".to_string(), "Пароль для регистрации:".to_string());
        map.insert("auto_register_twice".to_string(), "Вводить пароль 2 раза:".to_string());
        map.insert("connection_delay".to_string(), "Задержка подключения (сек):".to_string());
        map.insert("minecraft_version".to_string(), "Версия Minecraft:".to_string());
        map
    }

    fn english_translations() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("app_title".to_string(), "Js Simple Botzz - Bot Manager".to_string());
        map.insert("bot_list".to_string(), "Bot List:".to_string());
        map.insert("add_bot".to_string(), "Add Bot".to_string());
        map.insert("generator".to_string(), "Bot Generator".to_string());
        map.insert("stop_all".to_string(), "Stop All".to_string());
        map.insert("settings".to_string(), "Bot Settings".to_string());
        map.insert("id".to_string(), "ID:".to_string());
        map.insert("name".to_string(), "Name:".to_string());
        map.insert("server".to_string(), "Server:".to_string());
        map.insert("port".to_string(), "Port:".to_string());
        map.insert("account_type".to_string(), "Account Type:".to_string());
        map.insert("offline".to_string(), "Offline".to_string());
        map.insert("online".to_string(), "Online (Microsoft)".to_string());
        map.insert("username".to_string(), "Username:".to_string());
        map.insert("password".to_string(), "Password:".to_string());
        map.insert("auto_login".to_string(), "Auto-login".to_string());
        map.insert("auto_messages".to_string(), "Auto-messages on join:".to_string());
        map.insert("script_type".to_string(), "Script Type:".to_string());
        map.insert("auto_generation".to_string(), "Auto-generation".to_string());
        map.insert("built_in_editor".to_string(), "Built-in editor".to_string());
        map.insert("external_file".to_string(), "External file".to_string());
        map.insert("script_editor".to_string(), "Script Editor".to_string());
        map.insert("open_editor".to_string(), "Open Editor".to_string());
        map.insert("regenerate".to_string(), "Regenerate".to_string());
        map.insert("enabled".to_string(), "Enabled".to_string());
        map.insert("status".to_string(), "Status".to_string());
        map.insert("start".to_string(), "Start".to_string());
        map.insert("stop".to_string(), "Stop".to_string());
        map.insert("save".to_string(), "Save".to_string());
        map.insert("delete".to_string(), "Delete".to_string());
        map.insert("console".to_string(), "Console".to_string());
        map.insert("update".to_string(), "Update".to_string());
        map.insert("clear".to_string(), "Clear".to_string());
        map.insert("close".to_string(), "Close".to_string());
        map.insert("apply".to_string(), "Apply".to_string());
        map.insert("cancel".to_string(), "Cancel".to_string());
        map.insert("delete_script".to_string(), "Delete Script".to_string());
        map.insert("total_bots".to_string(), "Total bots".to_string());
        map.insert("default_server".to_string(), "Default server".to_string());
        map.insert("default_port".to_string(), "Default port".to_string());
        map.insert("add_message".to_string(), "Add message".to_string());
        map.insert("automation".to_string(), "Automation".to_string());
        map.insert("prompt_placeholder".to_string(), "Describe bot (example: 'Create bot with username Steve for server 192.168.1.1:25565, offline mode, auto-login')".to_string());
        map.insert("generate".to_string(), "Generate".to_string());
        map.insert("server_check".to_string(), "Server Check".to_string());
        map.insert("config_check".to_string(), "Config Check".to_string());
        map.insert("language".to_string(), "Language".to_string());
        map.insert("connection_params".to_string(), "Connection parameters:".to_string());
        map.insert("script_auto_gen".to_string(), "Script is auto-generated based on parameters".to_string());
        map.insert("script_path".to_string(), "Script path:".to_string());
        map.insert("not_specified".to_string(), "Not specified".to_string());
        map.insert("select_bot".to_string(), "Select a bot from the list on the left".to_string());
        map.insert("edit_script".to_string(), "Edit bot script:".to_string());
        map.insert("console_output".to_string(), "Bot console output:".to_string());
        map.insert("server_check_result".to_string(), "Server check result:".to_string());
        map.insert("server_available".to_string(), "Server available (ping: {}ms)".to_string());
        map.insert("server_unavailable".to_string(), "Server unavailable: {}".to_string());
        map.insert("config_check_result".to_string(), "Configuration check result:".to_string());
        map.insert("config_valid".to_string(), "Configuration is valid".to_string());
        map.insert("select_file".to_string(), "Select file".to_string());
        map.insert("enabled_status".to_string(), "Enabled: {}".to_string());
        map.insert("status_label".to_string(), "Status: {}".to_string());
        map.insert("logs".to_string(), "Logs".to_string());
        map.insert("bot_logs".to_string(), "Bot logs".to_string());
        map.insert("refresh".to_string(), "Refresh".to_string());
        map.insert("export".to_string(), "Export".to_string());
        map.insert("auto_password".to_string(), "Auto-login password:".to_string());
        map.insert("ai_settings".to_string(), "AI Settings".to_string());
        map.insert("discord_settings".to_string(), "Discord Settings".to_string());
        map.insert("packet_settings".to_string(), "Packet Settings".to_string());
        map.insert("api_key".to_string(), "API key:".to_string());
        map.insert("ai_model".to_string(), "Model:".to_string());
        map.insert("ai_provider".to_string(), "Provider:".to_string());
        map.insert("max_tokens".to_string(), "Max tokens:".to_string());
        map.insert("temperature".to_string(), "Temperature:".to_string());
        map.insert("system_prompt".to_string(), "System prompt:".to_string());
        map.insert("enable_ai".to_string(), "Enable AI".to_string());
        map.insert("discord_enabled".to_string(), "Discord Rich Presence:".to_string());
        map.insert("rate_limit".to_string(), "Rate limiting:".to_string());
        map.insert("packet_filter".to_string(), "Packet filter:".to_string());
        map.insert("min_interval".to_string(), "Min interval (ms):".to_string());
        map.insert("check_ai".to_string(), "Check AI".to_string());
        map.insert("ai_check_result".to_string(), "AI check result:".to_string());
        map.insert("theme_settings".to_string(), "Theme Settings".to_string());
        map.insert("background_color".to_string(), "Background color:".to_string());
        map.insert("panel_color".to_string(), "Panel color:".to_string());
        map.insert("text_color".to_string(), "Text color:".to_string());
        map.insert("accent_color".to_string(), "Accent color:".to_string());
        map.insert("button_color".to_string(), "Button color:".to_string());
        map.insert("border_color".to_string(), "Border color:".to_string());
        map.insert("auto_register".to_string(), "Auto-register".to_string());
        map.insert("auto_register_password".to_string(), "Register password:".to_string());
        map.insert("auto_register_twice".to_string(), "Enter password twice:".to_string());
        map.insert("connection_delay".to_string(), "Connection delay (sec):".to_string());
        map.insert("minecraft_version".to_string(), "Minecraft version:".to_string());
        map
    }
}
