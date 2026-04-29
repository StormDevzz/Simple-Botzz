# JsSimpleBotzz

Менеджер ботов на Rust с поддержкой Node.js скриптов и графическим интерфейсом.

## Возможности

- Запуск нескольких ботов одновременно
- Конфигурация сервера и порта для каждого бота
- Поддержка собственных Node.js скриптов
- Графический интерфейс для управления ботами
- Отдельные версии для Linux и Windows
- Передача переменных окружения в скрипты ботов

## Требования

- Rust (для компиляции и запуска менеджера)
- Node.js (для выполнения скриптов ботов)

## Структура проекта

```
JsSimpleBotzz/
├── Cargo.toml              # Конфигурация Rust проекта (CLI версия)
├── config.toml             # Конфигурация ботов
├── src/
│   └── main.rs            # CLI версия на Rust
├── platforms/
│   ├── linux/             # GUI версия для Linux
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── windows/           # GUI версия для Windows
│       ├── Cargo.toml
│       └── src/main.rs
├── scripts/               # Директория для Node.js скриптов ботов
│   ├── bot1.js            # Пример простого бота
│   └── bot2.js            # Пример чат-бота
├── assets/
│   └── icon.png           # Иконка приложения
└── README.md              # Этот файл
```

## Установка

1. Установите Rust: https://www.rust-lang.org/tools/install
2. Установите Node.js: https://nodejs.org/

## Использование

### CLI версия (консольная)

Запуск CLI версии:

```bash
cargo run
```

Или соберите релиз:

```bash
cargo build --release
./target/release/js_simple_botzz
```

### GUI версия (графический интерфейс)

#### Linux

Перейдите в директорию Linux версии:

```bash
cd platforms/linux
```

Запустите GUI версию:

```bash
cargo run
```

Или соберите релиз:

```bash
cargo build --release
./target/release/js_simple_botzz_linux
```

#### Windows

Перейдите в директорию Windows версии:

```cmd
cd platforms\windows
```

Запустите GUI версию:

```cmd
cargo run
```

Или соберите релиз:

```cmd
cargo build --release
.\target\release\js_simple_botzz_windows.exe
```

### Настройка конфигурации

Отредактируйте файл `config.toml`:

```toml
# Сервер по умолчанию для всех ботов
server = "localhost"

# Порт по умолчанию для всех ботов
port = 8080

# Список ботов
[[bots]]
id = "bot1"
name = "TestBot"
script_path = "scripts/bot1.js"
server = "localhost"
port = 8080
enabled = true
```

Или используйте GUI интерфейс для настройки ботов.

### Создание скрипта бота

Создайте Node.js скрипт в директории `scripts/`. Доступные переменные окружения:

- `BOT_ID` - уникальный идентификатор бота
- `BOT_NAME` - имя бота
- `SERVER` - сервер для подключения
- `PORT` - порт для подключения

Пример скрипта:

```javascript
const botId = process.env.BOT_ID;
const botName = process.env.BOT_NAME;
const server = process.env.SERVER;
const port = process.env.PORT;

console.log(`Запуск бота ${botName} на ${server}:${port}`);
```

## Управление ботами

### Через GUI интерфейс

- Выберите бота из списка слева
- Измените параметры (ID, имя, скрипт, сервер, порт)
- Нажмите "Запустить" для запуска бота
- Нажмите "Остановить" для остановки бота
- Нажмите "Сохранить" для сохранения изменений
- Используйте checkbox "Включен" для включения/выключения бота

### Через конфигурационный файл

- Включите/выключите бота через параметр `enabled` в `config.toml`
- Измените сервер и порт для каждого бота индивидуально
- Загружайте свои скрипты в директорию `scripts/` и укажите путь в конфигурации

## Примеры скриптов

- `scripts/bot1.js` - Простой бот с базовым циклом
- `scripts/bot2.js` - Чат-бот с обработкой сообщений

## Лицензия

MIT
# Simple-Botzz
# Simple-Botzz
