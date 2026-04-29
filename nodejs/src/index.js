import { BotManager } from './bot-manager.js';
import { ConfigLoader } from './config-loader.js';
import { Logger } from './logger.js';

// Основной файл для запуска Node.js интеграции
async function main() {
    const logger = new Logger('Main');
    logger.info('Starting Js Simple Botzz Node.js Integration...');

    try {
        // Загружаем конфигурацию
        const config = await ConfigLoader.load('./config.json');
        logger.info(`Loaded configuration for ${config.bots.length} bots`);

        // Создаем менеджер ботов
        const botManager = new BotManager(config);

        // Запускаем всех ботов
        await botManager.startAll();
        logger.info('All bots started successfully');

        // Обработка завершения
        process.on('SIGINT', async () => {
            logger.info('Shutting down...');
            await botManager.stopAll();
            process.exit(0);
        });

    } catch (error) {
        logger.error(`Failed to start: ${error.message}`);
        process.exit(1);
    }
}

main();
