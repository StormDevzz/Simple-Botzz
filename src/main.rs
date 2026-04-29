use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use std::fs;
use tokio::time::{sleep, Duration};

#[derive(Debug, Deserialize, Serialize)]
struct BotConfig {
    id: String,
    name: String,
    script_path: String,
    server: String,
    port: u16,
    enabled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    server: String,
    port: u16,
    bots: Vec<BotConfig>,
}

fn load_config(path: &str) -> Result<Config> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

async fn run_bot(bot: &BotConfig) -> Result<()> {
    println!("Запуск бота: {} (ID: {})", bot.name, bot.id);
    println!("Сервер: {}, Порт: {}", bot.server, bot.port);
    println!("Скрипт: {}", bot.script_path);

    let script_path = PathBuf::from(&bot.script_path);
    
    if !script_path.exists() {
        anyhow::bail!("Скрипт не найден: {}", bot.script_path);
    }

    let output = Command::new("node")
        .arg(&bot.script_path)
        .env("BOT_ID", &bot.id)
        .env("BOT_NAME", &bot.name)
        .env("SERVER", &bot.server)
        .env("PORT", bot.port.to_string())
        .output()?;

    if output.status.success() {
        println!("Бот {} успешно запущен", bot.name);
        println!("Вывод: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Ошибка запуска бота {}: {}", bot.name, String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("JsSimpleBotzz - Менеджер ботов");
    println!("================================");

    let config_path = "config.toml";
    let config = load_config(config_path)?;

    println!("Загружена конфигурация:");
    println!("Сервер по умолчанию: {}", config.server);
    println!("Порт по умолчанию: {}", config.port);
    println!("Количество ботов: {}", config.bots.len());
    println!();

    for bot in &config.bots {
        if bot.enabled {
            if let Err(e) = run_bot(bot).await {
                eprintln!("Ошибка при запуске бота {}: {}", bot.name, e);
            }
        } else {
            println!("Бот {} отключен в конфигурации", bot.name);
        }
        sleep(Duration::from_millis(500)).await;
    }

    println!("\nВсе боты запущены. Нажмите Ctrl+C для остановки.");

    // Держим программу запущенной
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}
