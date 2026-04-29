use crate::bots::bot_config::{BotConfig, AccountType};

/// Генерирует скрипт бота на основе параметров конфигурации
pub fn generate_script_from_params(bot: &BotConfig) -> String {
    let account_section = match bot.account_type {
        AccountType::Offline => {
            format!(r#"// Оффлайн режим
const username = "{}";
const auth = 'offline';
"#, bot.username)
        }
        AccountType::Online => {
            format!(r#"// Онлайн режим с лицензией
const username = "{}";
const password = "{}";
const auth = 'microsoft';
"#, bot.username, bot.password)
        }
    };

    let auto_login_section = if bot.auto_login {
        r#"// Авто-логин включен
function autoLogin() {
    console.log("Авто-логин...");
    // Логика авто-логина для Minecraft сервера
    connectToServer();
}
"#
    } else {
        "// Авто-логин отключен\n"
    };

    let auto_messages_section = if !bot.auto_messages.is_empty() {
        let messages = bot.auto_messages.iter()
            .map(|m| format!("    sendChatMessage('{}');", m))
            .collect::<Vec<_>>()
            .join("\n");
        format!(r#"// Авто-сообщения при входе
function sendAutoMessages() {{
{}
}}
"#, messages)
    } else {
        "// Авто-сообщения не настроены\n".to_string()
    };

    format!(r#"// Автоматически сгенерированный скрипт для бота: {}
// Переменные окружения:
// BOT_ID - {}
// BOT_NAME - {}
// SERVER - {}
// PORT - {}
// USERNAME - {}
// PASSWORD - {}
// ACCOUNT_TYPE - {:?}

const botId = process.env.BOT_ID;
const botName = process.env.BOT_NAME;
const server = process.env.SERVER;
const port = process.env.PORT;
const username = process.env.USERNAME;

console.log(`=== Запуск бота ${{botName}} ===`);
console.log(`ID: ${{botId}}`);
console.log(`Сервер: ${{server}}:${{port}}`);
console.log(`Пользователь: ${{username}}`);
console.log('===========================');

{}
// Подключение к серверу Minecraft
function connectToServer() {{
    console.log(`Подключение к серверу ${{server}}:${{port}}...`);
    
    // Здесь должна быть реальная логика подключения к Minecraft серверу
    // Используйте библиотеку mineflayer или подобную
    
    // Авто-логин
    {}
    
    console.log("Подключение к серверу...");
}}

// Отправка сообщения в чат
function sendChatMessage(message) {{
    console.log(`[Чат] ${{username}}: ${{message}}`);
    // Реальная отправка сообщения на сервер
}}

// Основной цикл бота
function botLoop() {{
    const timestamp = new Date().toISOString();
    console.log(`[${{timestamp}}] Бот ${{botName}} работает...`);
    
    // Ваша логика здесь
}}

// Запуск
connectToServer();
{}
setInterval(botLoop, 5000);

// Обработка завершения работы
process.on('SIGINT', () => {{
    console.log(`\nБот ${{botName}} останавливается...`);
    process.exit(0);
}});

console.log('Бот успешно инициализирован!');
"#, bot.name, bot.id, bot.name, bot.server, bot.port, bot.username, "***", bot.account_type, account_section, auto_login_section, auto_messages_section)
}

/// Возвращает базовый шаблон скрипта
pub fn get_default_script() -> String {
    r#"// Шаблон скрипта бота
// Переменные окружения:
// BOT_ID - уникальный идентификатор бота
// BOT_NAME - имя бота
// SERVER - сервер для подключения
// PORT - порт для подключения
// USERNAME - имя пользователя
// PASSWORD - пароль
// ACCOUNT_TYPE - тип аккаунта

const botId = process.env.BOT_ID;
const botName = process.env.BOT_NAME;
const server = process.env.SERVER;
const port = process.env.PORT;
const username = process.env.USERNAME;
const password = process.env.PASSWORD;
const accountType = process.env.ACCOUNT_TYPE;

console.log(`=== Запуск бота ${{botName}} ===`);
console.log(`ID: ${{botId}}`);
console.log(`Сервер: ${{server}}:${{port}}`);
console.log(`Пользователь: ${{username}}`);
console.log('===========================');

// Здесь ваша логика бота
function botLoop() {
    const timestamp = new Date().toISOString();
    console.log(`[${{timestamp}}] Бот ${{botName}} работает...`);
    
    // Ваша логика здесь
}

// Запуск основного цикла бота
setInterval(botLoop, 5000);

// Обработка завершения работы
process.on('SIGINT', () => {
    console.log(`\nБот ${{botName}} останавливается...`);
    process.exit(0);
});

console.log('Бот успешно инициализирован!');
"#.to_string()
}
