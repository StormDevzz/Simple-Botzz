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
        format!(r#"// Авто-логин включен
function autoLogin() {{
    console.log("Авто-логин...");
    sendChatMessage('/login {}');
}}
"#, bot.auto_login_password)
    } else {
        "// Авто-логин отключен\n".to_string()
    };

    let auto_register_section = if bot.auto_register {
        let register_cmd = if bot.auto_register_twice {
            format!(r#"// Авто-регистрация (пароль 2 раза)
function autoRegister() {{
    console.log("Авто-регистрация...");
    sendChatMessage('/register {}');
    sendChatMessage('/register {}');
}}
"#, bot.auto_register_password, bot.auto_register_password)
        } else {
            format!(r#"// Авто-регистрация (пароль 1 раз)
function autoRegister() {{
    console.log("Авто-регистрация...");
    sendChatMessage('/register {}');
}}
"#, bot.auto_register_password)
        };
        register_cmd
    } else {
        "// Авто-регистрация отключена\n".to_string()
    };

    let connection_delay_section = if bot.connection_delay > 0 {
        format!(r#"// Задержка перед подключением: {} сек
setTimeout(() => {{
    connectToServer();
}}, {} * 1000);
"#, bot.connection_delay, bot.connection_delay)
    } else {
        "// Без задержки подключения\n".to_string()
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
// MINECRAFT_VERSION - {}

const mineflayer = require('mineflayer');
const botId = process.env.BOT_ID;
const botName = process.env.BOT_NAME;
const server = process.env.SERVER;
const port = parseInt(process.env.PORT);
const username = process.env.USERNAME;
const password = process.env.PASSWORD;
const accountType = process.env.ACCOUNT_TYPE;
const minecraftVersion = process.env.MINECRAFT_VERSION || '1.20.4';

console.log(`=== Запуск бота ${{botName}} ===`);
console.log(`ID: ${{botId}}`);
console.log(`Сервер: ${{server}}:${{port}}`);
console.log(`Пользователь: ${{username}}`);
console.log(`Версия: ${{minecraftVersion}}`);
console.log('===========================');

{}

// Создаем клиента mineflayer
const bot = mineflayer.createBot({{
    host: server,
    port: port,
    username: username,
    password: password,
    auth: accountType === 'Online' ? 'microsoft' : 'offline',
    version: minecraftVersion,
}});

bot.on('connect', () => {{
    console.log('[OK] Подключено к серверу!');
}});

bot.on('login', () => {{
    console.log('[OK] Успешный вход на сервер!');
    
    // Авто-регистрация
    {}
    
    // Авто-логин
    {}
    
    // Авто-сообщения
    {}
}});

bot.on('error', (err) => {{
    console.error('[ERROR]', err);
}});

bot.on('end', () => {{
    console.log('[INFO] Соединение закрыто');
}});

bot.on('kicked', (reason) => {{
    console.log('[KICKED]', reason);
}});

// Отправка сообщения в чат
function sendChatMessage(message) {{
    if (bot) {{
        bot.chat(message);
    }}
}}

// Основной цикл бота
setInterval(() => {{
    const timestamp = new Date().toISOString();
    console.log(`[${{timestamp}}] Бот ${{botName}} работает...`);
}}, 5000);

// Обработка завершения работы
process.on('SIGINT', () => {{
    console.log(`\nБот ${{botName}} останавливается...`);
    if (bot) {{
        bot.quit();
    }}
    process.exit(0);
}});

console.log('Бот успешно инициализирован!');
"#, bot.name, bot.id, bot.name, bot.server, bot.port, bot.username, "***", bot.account_type, bot.minecraft_version, account_section, 
if bot.auto_register {
    format!("    setTimeout(() => {{\n        sendChatMessage('/register {}');\n        {}\n    }}, 1000);", 
        bot.auto_register_password,
        if bot.auto_register_twice {
            format!("        sendChatMessage('/register {}');", bot.auto_register_password)
        } else {
            String::new()
        })
    } else {
        String::new()
    },
if bot.auto_login {
    format!("    setTimeout(() => {{\n        sendChatMessage('/login {}');\n    }}, 2000);", bot.auto_login_password)
} else {
    String::new()
},
if !bot.auto_messages.is_empty() {
    let msgs = bot.auto_messages.iter()
        .map(|m| format!("        sendChatMessage('{}');", m))
        .collect::<Vec<_>>()
        .join("\n");
    format!("    setTimeout(() => {{\n{}\n    }}, 3000);", msgs)
} else {
    String::new()
})
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
