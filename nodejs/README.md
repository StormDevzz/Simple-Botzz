# Js Simple Botzz - Node.js Integration

Node.js интеграция для Js Simple Botzz с использованием mineflayer.

## Установка

```bash
cd nodejs
npm install
```

## Использование

```bash
npm start
```

## Конфигурация

Отредактируйте `config.json` для настройки ботов:

```json
{
  "server": "localhost",
  "port": 25565,
  "bots": [
    {
      "id": "bot1",
      "name": "MyBot",
      "server": "localhost",
      "port": 25565,
      "accountType": "offline",
      "username": "BotName",
      "password": "",
      "autoLogin": false,
      "autoLoginPassword": "",
      "autoMessages": ["Hello!"],
      "enabled": true
    }
  ]
}
```

## Возможности

- **Mineflayer**: Полноценный Minecraft бот
- **Pathfinding**: Поиск пути с prismarine-pathfinder
- **Packet Management**: Управление пакетами для античита
- **Auto-login**: Автоматический вход на сервер
- **Auto-messages**: Автоматические сообщения при входе

## Структура

- `src/index.js` - Точка входа
- `src/bot-manager.js` - Менеджер ботов
- `src/bot.js` - Класс бота
- `src/config-loader.js` - Загрузчик конфигурации
- `src/logger.js` - Логгер
- `src/packet-manager.js` - Менеджер пакетов
