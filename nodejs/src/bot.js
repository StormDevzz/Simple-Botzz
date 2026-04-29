import mineflayer from 'mineflayer';
import { pathfinder, Movements, goals } from 'prismarine-pathfinder';
import { Logger } from './logger.js';
import { PacketManager } from './packet-manager.js';

/**
 * Класс бота Minecraft
 */
export class Bot {
    constructor(config) {
        this.config = config;
        this.client = null;
        this.logger = new Logger(`Bot-${config.id}`);
        this.packetManager = new PacketManager();
        this.isConnected = false;
    }

    /**
     * Подключается к серверу
     */
    async connect() {
        const options = {
            host: this.config.server,
            port: this.config.port,
            username: this.config.username,
            auth: this.config.accountType === 'online' ? 'microsoft' : 'offline',
            password: this.config.password || undefined,
            version: false // Автоопределение версии
        };

        this.logger.info(`Connecting to ${this.config.server}:${this.config.port} as ${this.config.username}`);

        try {
            this.client = mineflayer.createBot(options);

            this.setupEventHandlers();
            this.setupPathfinding();

            await new Promise((resolve, reject) => {
                this.client.once('spawn', resolve);
                this.client.once('error', reject);
                this.client.once('kicked', (reason) => reject(new Error(`Kicked: ${reason}`)));
            });

            this.isConnected = true;
            this.logger.info('Connected successfully');

            // Авто-логин если включен
            if (this.config.autoLogin && this.config.autoLoginPassword) {
                await this.autoLogin();
            }

            // Авто-сообщения
            if (this.config.autoMessages && this.config.autoMessages.length > 0) {
                this.sendAutoMessages();
            }

        } catch (error) {
            this.logger.error(`Connection failed: ${error.message}`);
            throw error;
        }
    }

    /**
     * Настраивает обработчики событий
     */
    setupEventHandlers() {
        this.client.on('chat', (username, message) => {
            this.logger.chat(`[Chat] ${username}: ${message}`);
        });

        this.client.on('error', (err) => {
            this.logger.error(`Error: ${err.message}`);
        });

        this.client.on('kicked', (reason) => {
            this.logger.warn(`Kicked: ${reason}`);
            this.isConnected = false;
        });

        this.client.on('end', () => {
            this.logger.info('Connection ended');
            this.isConnected = false;
        });

        // Управление пакетами для античита
        this.client.on('packet', (data, meta) => {
            this.packetManager.recordPacket(meta.name, 'incoming');
        });
    }

    /**
     * Настраивает pathfinding
     */
    setupPathfinding() {
        this.client.loadPlugin(pathfinder);
        
        const defaultMove = new Movements(this.client);
        this.client.pathfinder.setMovements(defaultMove);
    }

    /**
     * Авто-логин на сервер
     */
    async autoLogin() {
        this.logger.info('Performing auto-login...');
        // TODO: Реализовать логику авто-логина для конкретного сервера
        // Обычно это /login <password>
        await this.chat(`/login ${this.config.autoLoginPassword}`);
    }

    /**
     * Отправляет авто-сообщения
     */
    sendAutoMessages() {
        setTimeout(async () => {
            for (const message of this.config.autoMessages) {
                await this.chat(message);
                await new Promise(r => setTimeout(r, 1000)); // Задержка между сообщениями
            }
        }, 2000); // Задержка после подключения
    }

    /**
     * Отправляет сообщение в чат
     */
    async chat(message) {
        if (this.isConnected && this.client) {
            this.client.chat(message);
            this.packetManager.recordPacket('chat', 'outgoing');
        }
    }

    /**
     * Перемещается к цели
     */
    async goTo(x, y, z) {
        if (!this.isConnected || !this.client) return;

        const goal = new goals.GoalBlock(x, y, z);
        this.client.pathfinder.setGoal(goal);
    }

    /**
     * Отключается от сервера
     */
    async disconnect() {
        if (this.client) {
            this.client.quit();
            this.isConnected = false;
            this.logger.info('Disconnected');
        }
    }

    /**
     * Проверяет подключение
     */
    isConnected() {
        return this.isConnected;
    }

    /**
     * Получает имя пользователя
     */
    getUsername() {
        return this.config.username;
    }

    /**
     * Получает сервер
     */
    getServer() {
        return `${this.config.server}:${this.config.port}`;
    }
}
