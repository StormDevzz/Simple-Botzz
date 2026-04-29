/// Шаблон скрипта для Minecraft бота с базовым функционалом
pub fn get_minecraft_bot_template() -> String {
    r#"// Minecraft Bot Template
// Для работы требуется библиотека mineflayer: npm install mineflayer

const mineflayer = require('mineflayer');

const botId = process.env.BOT_ID;
const botName = process.env.BOT_NAME;
const server = process.env.SERVER;
const port = parseInt(process.env.PORT);
const username = process.env.USERNAME;
const password = process.env.PASSWORD;
const accountType = process.env.ACCOUNT_TYPE;

console.log(`=== Запуск Minecraft бота ${botName} ===`);
console.log(`Сервер: ${server}:${port}`);
console.log(`Пользователь: ${username}`);

const bot = mineflayer.createBot({
    host: server,
    port: port,
    username: username,
    password: password,
    auth: accountType === 'Online' ? 'microsoft' : 'offline'
});

bot.on('login', () => {
    console.log(`${botName} успешно вошел на сервер!`);
});

bot.on('spawn', () => {
    console.log(`${botName} появился в мире!`);
    // Авто-сообщения при входе
    sendAutoMessages();
});

bot.on('chat', (username, message) => {
    console.log(`[Чат] ${username}: ${message}`);
    // Обработка сообщений чата
});

bot.on('error', (err) => {
    console.error(`Ошибка: ${err.message}`);
});

bot.on('end', () => {
    console.log(`${botName} отключился от сервера`);
});

function sendAutoMessages() {
    // Отправка авто-сообщений
    // bot.chat('Привет от бота!');
}

console.log('Бот инициализирован, ожидание подключения...');
"#.to_string()
}

/// Шаблон скрипта для чат-бота
pub fn get_chat_bot_template() -> String {
    r#"// Chat Bot Template
// Простой чат-бот с ответами на команды

const botId = process.env.BOT_ID;
const botName = process.env.BOT_NAME;
const server = process.env.SERVER;
const port = parseInt(process.env.PORT);
const username = process.env.USERNAME;

console.log(`=== Запуск чат-бота ${botName} ===`);

// Команды бота
const commands = {
    '!help': 'Доступные команды: !help, !time, !info',
    '!time': () => new Date().toLocaleString(),
    '!info': () => `Бот ${botName} v1.0`
};

function processCommand(message) {
    const command = message.toLowerCase().trim();
    
    if (commands[command]) {
        if (typeof commands[command] === 'function') {
            return commands[command]();
        }
        return commands[command];
    }
    
    return 'Неизвестная команда. Введите !help для справки.';
}

// Имитация чата
function chatLoop() {
    console.log(`[${new Date().toISOString()}] Чат-бот ${botName} готов к работе...`);
}

setInterval(chatLoop, 5000);

console.log('Чат-бот инициализирован!');
"#.to_string()
}
