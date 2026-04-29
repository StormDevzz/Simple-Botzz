use anyhow::Result;
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use std::fs;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct BotConfig {
    id: String,
    name: String,
    script_path: String,
    server: String,
    port: u16,
    enabled: bool,
    #[serde(default)]
    status: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    server: String,
    port: u16,
    bots: Vec<BotConfig>,
}

struct BotManagerApp {
    config: Config,
    config_path: String,
    selected_bot: Option<usize>,
    runtime: Arc<Mutex<Runtime>>,
    bot_processes: Arc<Mutex<Vec<Option<std::process::Child>>>>,
    add_icon: egui::TextureHandle,
    apply_icon: egui::TextureHandle,
    delete_icon: egui::TextureHandle,
    edit_icon: egui::TextureHandle,
}

impl BotManagerApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config_path = "config.toml".to_string();
        let config = Self::load_config(&config_path).unwrap_or_else(|_| Config {
            server: "localhost".to_string(),
            port: 8080,
            bots: vec![],
        });

        let runtime = Arc::new(Mutex::new(
            Runtime::new().expect("Failed to create Tokio runtime")
        ));

        // Load icons
        let add_icon = load_icon(cc, "../../assets/add.png");
        let apply_icon = load_icon(cc, "../../assets/apply.png");
        let delete_icon = load_icon(cc, "../../assets/delete.png");
        let edit_icon = load_icon(cc, "../../assets/edit.png");

        Self {
            config,
            config_path,
            selected_bot: None,
            runtime,
            bot_processes: Arc::new(Mutex::new(vec![])),
            add_icon,
            apply_icon,
            delete_icon,
            edit_icon,
        }
    }

    fn load_config(path: &str) -> Result<Config> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    fn save_config(&self) -> Result<()> {
        let content = toml::to_string_pretty(&self.config)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }

    fn run_bot(&self, bot: &BotConfig) -> Result<()> {
        let script_path = PathBuf::from(&bot.script_path);
        
        if !script_path.exists() {
            anyhow::bail!("Скрипт не найден: {}", bot.script_path);
        }

        let child = Command::new("node")
            .arg(&bot.script_path)
            .env("BOT_ID", &bot.id)
            .env("BOT_NAME", &bot.name)
            .env("SERVER", &bot.server)
            .env("PORT", bot.port.to_string())
            .spawn()?;

        Ok(child)
    }

    fn stop_bot(&self, index: usize) {
        let mut processes = self.bot_processes.lock().unwrap();
        if let Some(Some(mut child)) = processes.get_mut(index) {
            let _ = child.kill();
            *processes.get_mut(index).unwrap() = None;
        }
    }

    fn delete_bot(&mut self, index: usize) {
        self.stop_bot(index);
        self.config.bots.remove(index);
        if self.selected_bot == Some(index) {
            self.selected_bot = None;
        } else if self.selected_bot > Some(index) {
            self.selected_bot = self.selected_bot.map(|i| i - 1);
        }
        let _ = self.save_config();
    }
}

fn load_icon(cc: &eframe::CreationContext<'_>, path: &str) -> egui::TextureHandle {
    let image_data = include_bytes!(path);
    let image = image::load_from_memory(image_data).expect("Failed to load icon");
    let image_buffer = image.to_rgba8();
    let size = [image_buffer.width() as usize, image_buffer.height() as usize];
    
    cc.egui_ctx.load_texture(
        path,
        egui::ColorImage::from_rgba_unmultiplied(size, &image_buffer),
        egui::TextureOptions::LINEAR,
    )
}

impl eframe::App for BotManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.heading("JsSimpleBotzz - Менеджер Ботов (Windows)");
            ui.separator();
        });

        egui::SidePanel::left("bot_list").width(250.0).show(ctx, |ui| {
            ui.label("Список ботов:");
            ui.separator();

            for (index, bot) in self.config.bots.iter().enumerate() {
                let selected = self.selected_bot == Some(index);
                if ui.selectable_label(selected, &bot.name).clicked() {
                    self.selected_bot = Some(index);
                }
            }

            ui.separator();
            if ui.add(egui::ImageButton::new(self.add_icon.sized(egui::vec2(24.0, 24.0)))).on_hover_text("Добавить бота").clicked() {
                let new_bot = BotConfig {
                    id: format!("bot{}", self.config.bots.len() + 1),
                    name: format!("NewBot{}", self.config.bots.len() + 1),
                    script_path: "scripts\\new_bot.js".to_string(),
                    server: self.config.server.clone(),
                    port: self.config.port,
                    enabled: true,
                    status: "Остановлен".to_string(),
                };
                self.config.bots.push(new_bot);
                let _ = self.save_config();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(index) = self.selected_bot {
                let bot = &mut self.config.bots[index];
                
                ui.group(|ui| {
                    ui.heading(format!("Настройки бота: {}", bot.name));
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("ID:");
                        ui.text_edit_singleline(&mut bot.id);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Имя:");
                        ui.text_edit_singleline(&mut bot.name);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Скрипт:");
                        ui.text_edit_singleline(&mut bot.script_path);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Сервер:");
                        ui.text_edit_singleline(&mut bot.server);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Порт:");
                        ui.add(egui::DragValue::new(&mut bot.port).range(1..=65535));
                    });

                    ui.checkbox(&mut bot.enabled, "Включен");
                    
                    ui.separator();
                    ui.label(format!("Статус: {}", bot.status));

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.add(egui::ImageButton::new(self.edit_icon.sized(egui::vec2(24.0, 24.0)))).on_hover_text("Запустить").clicked() {
                            match self.run_bot(bot) {
                                Ok(child) => {
                                    bot.status = "Запущен".to_string();
                                    let mut processes = self.bot_processes.lock().unwrap();
                                    while processes.len() <= index {
                                        processes.push(None);
                                    }
                                    processes[index] = Some(child);
                                }
                                Err(e) => {
                                    bot.status = format!("Ошибка: {}", e);
                                }
                            }
                        }

                        if ui.button("Остановить").clicked() {
                            self.stop_bot(index);
                            bot.status = "Остановлен".to_string();
                        }

                        if ui.add(egui::ImageButton::new(self.apply_icon.sized(egui::vec2(24.0, 24.0)))).on_hover_text("Сохранить").clicked() {
                            let _ = self.save_config();
                        }

                        if ui.add(egui::ImageButton::new(self.delete_icon.sized(egui::vec2(24.0, 24.0)))).on_hover_text("Удалить бота").clicked() {
                            self.delete_bot(index);
                        }
                    });
                });
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Выберите бота из списка слева");
                });
            }
        });

        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!("Сервер по умолчанию: {}", self.config.server));
                ui.label(format!("Порт по умолчанию: {}", self.config.port));
                ui.label(format!("Всего ботов: {}", self.config.bots.len()));
            });
        });
    }
}

fn main() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../../assets/Minecraft.png")[..])
                    .unwrap_or_default(),
            ),
        ..Default::default()
    };

    eframe::run_native(
        "JsSimpleBotzz - Windows",
        options,
        Box::new(|cc| Ok(Box::new(BotManagerApp::new(cc)))),
    )?;

    Ok(())
}
