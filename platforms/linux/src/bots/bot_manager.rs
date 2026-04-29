use anyhow::Result;
use super::bot_config::BotConfig;
use std::path::PathBuf;
use std::process::{Command, Stdio, Child};
use std::sync::{Arc, Mutex};
use std::io::{BufRead, BufReader};

/// Процесс бота с отслеживанием вывода
pub struct BotProcess {
    pub child: Option<Child>,
    pub output_lines: Arc<Mutex<Vec<String>>>,
}

impl BotProcess {
    /// Создает новый процесс бота
    pub fn new() -> Self {
        Self {
            child: None,
            output_lines: Arc::new(Mutex::new(vec![])),
        }
    }

    /// Запускает бота с заданной конфигурацией
    pub fn start(&mut self, bot: &BotConfig, data_dir: &str) -> Result<()> {
        let script_path = if bot.use_external_script {
            if let Some(ref path) = bot.script_path {
                PathBuf::from(path)
            } else {
                anyhow::bail!("Внешний скрипт не указан");
            }
        } else {
            let temp_script_path = format!("{}/scripts/{}.js", data_dir, bot.id);
            std::fs::write(&temp_script_path, &bot.script_content)?;
            PathBuf::from(&temp_script_path)
        };

        if !script_path.exists() {
            anyhow::bail!("Скрипт не найден: {:?}", script_path);
        }

        let output_lines = self.output_lines.clone();
        let bot_name = bot.name.clone();
        let bot_name2 = bot_name.clone();

        let mut child = Command::new("node")
            .arg(&script_path)
            .env("BOT_ID", &bot.id)
            .env("BOT_NAME", &bot.name)
            .env("SERVER", &bot.server)
            .env("PORT", bot.port.to_string())
            .env("USERNAME", &bot.username)
            .env("PASSWORD", &bot.password)
            .env("ACCOUNT_TYPE", format!("{:?}", bot.account_type))
            .env("MINECRAFT_VERSION", &bot.minecraft_version)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().expect("Failed to take stdout");
        let stderr = child.stderr.take().expect("Failed to take stderr");

        // Чтение stdout
        let output_lines_clone = output_lines.clone();
        std::thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let mut lines = output_lines_clone.lock().unwrap();
                    lines.push(format!("[{}] {}", bot_name, line));
                    if lines.len() > 1000 {
                        lines.remove(0);
                    }
                }
            }
        });

        // Чтение stderr
        let output_lines_clone2 = output_lines.clone();
        std::thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let mut lines = output_lines_clone2.lock().unwrap();
                    lines.push(format!("[{}] ERROR: {}", bot_name2, line));
                    if lines.len() > 1000 {
                        lines.remove(0);
                    }
                }
            }
        });

        self.child = Some(child);
        Ok(())
    }

    /// Останавливает процесс бота
    pub fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }

    /// Проверяет, запущен ли процесс
    pub fn is_running(&mut self) -> bool {
        if let Some(ref mut child) = self.child {
            match child.try_wait() {
                Ok(Some(_)) => false,
                Ok(None) => true,
                Err(_) => false,
            }
        } else {
            false
        }
    }

    /// Возвращает строки вывода процесса
    pub fn get_output(&self) -> Vec<String> {
        self.output_lines.lock().unwrap().clone()
    }
}
