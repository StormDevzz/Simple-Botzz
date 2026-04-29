import net from 'net';

/**
 * Типы сообщений для моста Rust-Node.js
 */
const MessageType = {
    BotStatus: 'BotStatus',
    StartBot: 'StartBot',
    StopBot: 'StopBot',
    RestartBot: 'RestartBot',
    GetLogs: 'GetLogs',
    UpdateConfig: 'UpdateConfig',
    Ping: 'Ping',
    Pong: 'Pong',
    Error: 'Error',
    Data: 'Data',
};

/**
 * Сообщение для моста
 */
class BridgeMessage {
    constructor(messageType, botId = null, data = {}) {
        this.message_type = messageType;
        this.bot_id = botId;
        this.data = data;
        this.timestamp = Math.floor(Date.now() / 1000);
    }

    toJSON() {
        return JSON.stringify(this);
    }

    static fromJSON(json) {
        const obj = JSON.parse(json);
        const message = new BridgeMessage(obj.message_type, obj.bot_id, obj.data);
        message.timestamp = obj.timestamp;
        return message;
    }
}

/**
 * IPC мост для общения с Rust
 */
class IPCBridge {
    constructor(port = 54321) {
        this.port = port;
        this.client = null;
        this.handlers = new Map();
        this.connected = false;
    }

    /**
     * Подключается к Rust серверу
     */
    connect() {
        return new Promise((resolve, reject) => {
            this.client = new net.Socket();
            
            this.client.connect(this.port, '127.0.0.1', () => {
                this.connected = true;
                console.log('[Bridge] Connected to Rust server');
                this.startListening();
                resolve();
            });

            this.client.on('error', (err) => {
                console.error('[Bridge] Connection error:', err);
                reject(err);
            });

            this.client.on('close', () => {
                this.connected = false;
                console.log('[Bridge] Disconnected from Rust server');
            });
        });
    }

    /**
     * Начинает слушать сообщения от Rust
     */
    startListening() {
        let buffer = '';
        
        this.client.on('data', (data) => {
            buffer += data.toString();
            
            const lines = buffer.split('\n');
            buffer = lines.pop() || '';
            
            for (const line of lines) {
                if (line.trim()) {
                    try {
                        const message = BridgeMessage.fromJSON(line);
                        this.handleMessage(message);
                    } catch (err) {
                        console.error('[Bridge] Failed to parse message:', err);
                    }
                }
            }
        });
    }

    /**
     * Обрабатывает входящее сообщение
     */
    handleMessage(message) {
        const handler = this.handlers.get(message.message_type);
        if (handler) {
            const response = handler(message);
            if (response) {
                this.send(response);
            }
        } else {
            console.log('[Bridge] No handler for message type:', message.message_type);
        }
    }

    /**
     * Добавляет обработчик сообщения
     */
    on(messageType, handler) {
        this.handlers.set(messageType, handler);
    }

    /**
     * Отправляет сообщение в Rust
     */
    send(message) {
        if (this.connected && this.client) {
            this.client.write(message.toJSON() + '\n');
        } else {
            console.error('[Bridge] Not connected to Rust server');
        }
    }

    /**
     * Отправляет пинг
     */
    ping() {
        const message = new BridgeMessage(MessageType.Ping);
        this.send(message);
    }

    /**
     * Отправляет статус бота
     */
    sendBotStatus(botId, status) {
        const message = new BridgeMessage(MessageType.BotStatus, botId, { status });
        this.send(message);
    }

    /**
     * Отправляет логи бота
     */
    sendLogs(botId, logs) {
        const message = new BridgeMessage(MessageType.GetLogs, botId, { logs });
        this.send(message);
    }

    /**
     * Отправляет ошибку
     */
    sendError(botId, error) {
        const message = new BridgeMessage(MessageType.Error, botId, { error });
        this.send(message);
    }

    /**
     * Отключается от сервера
     */
    disconnect() {
        if (this.client) {
            this.client.end();
            this.connected = false;
        }
    }
}

export { IPCBridge, BridgeMessage, MessageType };
