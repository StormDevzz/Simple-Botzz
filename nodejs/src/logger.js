/**
 * Логгер для Node.js интеграции
 */
export class Logger {
    constructor(prefix) {
        this.prefix = prefix;
    }

    /**
     * Форматирует сообщение с временной меткой
     */
    format(level, message) {
        const timestamp = new Date().toISOString();
        return `[${timestamp}] [${this.prefix}] [${level}] ${message}`;
    }

    /**
     * Логирует информационное сообщение
     */
    info(message) {
        console.log(this.format('INFO', message));
    }

    /**
     * Логирует предупреждение
     */
    warn(message) {
        console.warn(this.format('WARN', message));
    }

    /**
     * Логирует ошибку
     */
    error(message) {
        console.error(this.format('ERROR', message));
    }

    /**
     * Логирует сообщение чата
     */
    chat(message) {
        console.log(this.format('CHAT', message));
    }

    /**
     * Логирует отладочное сообщение
     */
    debug(message) {
        if (process.env.DEBUG) {
            console.log(this.format('DEBUG', message));
        }
    }
}
