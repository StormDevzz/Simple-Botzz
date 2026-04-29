/**
 * Менеджер пакетов для Node.js интеграции
 * Предотвращает флаги на античите
 */
export class PacketManager {
    constructor() {
        this.stats = {
            totalSent: 0,
            totalReceived: 0,
            byType: {}
        };
        this.lastPacketTime = Date.now();
        this.minInterval = 50; // 50мс между пакетами
        this.rateLimitEnabled = true;
    }

    /**
     * Регистрирует отправку пакета
     */
    recordPacket(type, direction) {
        const now = Date.now();
        
        if (direction === 'outgoing') {
            if (this.rateLimitEnabled && (now - this.lastPacketTime) < this.minInterval) {
                return false; // Пакет заблокирован rate limiter
            }
            this.stats.totalSent++;
            this.lastPacketTime = now;
        } else {
            this.stats.totalReceived++;
        }

        if (!this.stats.byType[type]) {
            this.stats.byType[type] = { sent: 0, received: 0 };
        }

        if (direction === 'outgoing') {
            this.stats.byType[type].sent++;
        } else {
            this.stats.byType[type].received++;
        }

        return true;
    }

    /**
     * Получает статистику
     */
    getStats() {
        return this.stats;
    }

    /**
     * Сбрасывает статистику
     */
    resetStats() {
        this.stats = {
            totalSent: 0,
            totalReceived: 0,
            byType: {}
        };
    }

    /**
     * Включает/выключает rate limiting
     */
    setRateLimit(enabled) {
        this.rateLimitEnabled = enabled;
    }

    /**
     * Устанавливает минимальный интервал
     */
    setMinInterval(ms) {
        this.minInterval = ms;
    }
}
