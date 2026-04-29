// Пример чат-бота для JsSimpleBotzz
// Этот бот демонстрирует более сложную логику

const botId = process.env.BOT_ID;
const botName = process.env.BOT_NAME;
const server = process.env.SERVER;
const port = process.env.PORT;

console.log(`=== Чат-бот ${botName} ===`);
console.log(`ID: ${botId}`);
console.log(`Сервер: ${server}:${port}`);
console.log('========================');

// Имитация чат-бота
class ChatBot {
    constructor(name, id) {
        this.name = name;
        this.id = id;
        this.messageCount = 0;
    }

    processMessage(message) {
        this.messageCount++;
        console.log(`[${new Date().toISOString()}] Получено сообщение #${this.messageCount}: ${message}`);
        
        // Пример простой логики ответа
        const responses = [
            "Привет! Я чат-бот.",
            "Как я могу помочь?",
            "Интересный вопрос!",
            "Дайте мне подумать...",
            "Я здесь, чтобы помочь вам!"
        ];
        
        const randomResponse = responses[Math.floor(Math.random() * responses.length)];
        console.log(`[${new Date().toISOString()}] Ответ: ${randomResponse}`);
        
        return randomResponse;
    }

    start() {
        console.log(`Чат-бот ${this.name} начал работу`);
        
        // Имитация входящих сообщений
        const sampleMessages = [
            "Привет",
            "Как дела?",
            "Помоги мне",
            "Что ты умеешь?",
            "Спасибо"
        ];
        
        let index = 0;
        setInterval(() => {
            if (index < sampleMessages.length) {
                this.processMessage(sampleMessages[index]);
                index++;
            } else {
                index = 0; // Повторяем сообщения
            }
        }, 3000);
    }

    stop() {
        console.log(`Чат-бот ${this.name} останавливается. Обработано сообщений: ${this.messageCount}`);
    }
}

// Создание и запуск бота
const bot = new ChatBot(botName, botId);
bot.start();

// Обработка завершения работы
process.on('SIGINT', () => {
    bot.stop();
    process.exit(0);
});
