import { Bot } from './bot.js';
import { Logger } from './logger.js';

/**
 * Менеджер ботов для управления множеством ботов
 */
export class BotManager {
    constructor(config) {
        this.config = config;
        this.bots = new Map();
        this.logger = new Logger('BotManager');
    }

    /**
     * Запускает всех ботов
     */
    async startAll() {
        for (const botConfig of this.config.bots) {
            await this.startBot(botConfig);
        }
    }

    /**
     * Запускает отдельного бота
     */
    async startBot(botConfig) {
        if (this.bots.has(botConfig.id)) {
            this.logger.warn(`Bot ${botConfig.id} is already running`);
            return;
        }

        try {
            const bot = new Bot(botConfig);
            await bot.connect();
            this.bots.set(botConfig.id, bot);
            this.logger.info(`Bot ${botConfig.id} started successfully`);
        } catch (error) {
            this.logger.error(`Failed to start bot ${botConfig.id}: ${error.message}`);
        }
    }

    /**
     * Останавливает всех ботов
     */
    async stopAll() {
        const stopPromises = Array.from(this.bots.values()).map(bot => bot.disconnect());
        await Promise.all(stopPromises);
        this.bots.clear();
        this.logger.info('All bots stopped');
    }

    /**
     * Останавливает отдельного бота
     */
    async stopBot(botId) {
        const bot = this.bots.get(botId);
        if (bot) {
            await bot.disconnect();
            this.bots.delete(botId);
            this.logger.info(`Bot ${botId} stopped`);
        }
    }

    /**
     * Получает статус всех ботов
     */
    getStatus() {
        const status = {};
        for (const [id, bot] of this.bots) {
            status[id] = {
                connected: bot.isConnected(),
                username: bot.getUsername(),
                server: bot.getServer()
            };
        }
        return status;
    }

    /**
     * Отправляет сообщение от имени бота
     */
    async sendMessage(botId, message) {
        const bot = this.bots.get(botId);
        if (bot) {
            await bot.chat(message);
        }
    }
}
