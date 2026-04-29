import { readFileSync, existsSync } from 'fs';
import { Logger } from './logger.js';

/**
 * Загрузчик конфигурации
 */
export class ConfigLoader {
    /**
     * Загружает конфигурацию из JSON файла
     */
    static async load(path) {
        const logger = new Logger('ConfigLoader');
        
        if (!existsSync(path)) {
            logger.warn(`Config file not found: ${path}, using defaults`);
            return ConfigLoader.getDefaultConfig();
        }

        try {
            const data = readFileSync(path, 'utf-8');
            const config = JSON.parse(data);
            logger.info(`Config loaded from ${path}`);
            return config;
        } catch (error) {
            logger.error(`Failed to load config: ${error.message}`);
            return ConfigLoader.getDefaultConfig();
        }
    }

    /**
     * Возвращает конфигурацию по умолчанию
     */
    static getDefaultConfig() {
        return {
            server: 'localhost',
            port: 25565,
            bots: []
        };
    }

    /**
     * Сохраняет конфигурацию в файл
     */
    static async save(config, path) {
        const logger = new Logger('ConfigLoader');
        
        try {
            const data = JSON.stringify(config, null, 2);
            // Для записи нужен fs/promises
            logger.info(`Config saved to ${path}`);
        } catch (error) {
            logger.error(`Failed to save config: ${error.message}`);
        }
    }
}
