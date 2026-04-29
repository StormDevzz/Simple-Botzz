mod bots;
mod scripts;
mod console;
mod utils;
mod generator;
mod checks;
mod lgs;
mod windows;
mod packets;
mod discord;
mod ai;
mod pathfinding;
mod colors;
mod bridge;

use anyhow::Result;
use eframe::egui;
use bots::{BotConfig, Config, AccountType, BotProcess};
use scripts::{get_default_script, generate_script_from_params};
use utils::{ensure_dir, load_toml, save_toml};
use generator::{parse_prompt, build_bot_from_prompt};
use checks::{check_server, validate_bot_config, check_ai_config, ServerCheckResult, ConfigCheckResult, AICheckResult};
use lgs::{Language, Translator};
use windows::{EditorWindow, ConsoleWindow, GeneratorWindow, LogsWindow};
use packets::PacketManager;
use discord::DiscordRichPresence;
use ai::{AIConfig, AIClient};
use colors::{ThemeManager, ColorPalette};
use bridge::IPCBridge;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;

struct BotManagerApp {
    config: Config,
    config_path: String,
    data_dir: String,
    selected_bot: Option<usize>,
    bot_processes: Arc<Mutex<Vec<BotProcess>>>,
    script_editor: String,
    script_editor_backup: String,
    show_script_editor: bool,
    show_console: bool,
    console_output: Vec<String>,
    show_generator: bool,
    generator_prompt: String,
    generator_status: String,
    generator_progress: f32,
    server_check_result: Option<ServerCheckResult>,
    config_check_result: Option<ConfigCheckResult>,
    ai_check_result: Option<AICheckResult>,
    translator: Translator,
    logs_window: LogsWindow,
    packet_manager: PacketManager,
    discord_rich_presence: DiscordRichPresence,
    ai_config: AIConfig,
    show_ai_settings: bool,
    show_discord_settings: bool,
    show_packet_settings: bool,
    theme_manager: ThemeManager,
    show_theme_settings: bool,
    ipc_bridge: IPCBridge,
}

impl BotManagerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let data_dir = ".js_simple_botzz".to_string();
        let config_path = format!("{}/config.toml", data_dir);
        
        // Создаем папку для данных
        ensure_dir(&data_dir).expect("Failed to create data directory");
        ensure_dir(&format!("{}/scripts", data_dir)).expect("Failed to create scripts directory");
        ensure_dir(&format!("{}/logs", data_dir)).expect("Failed to create logs directory");
        
        let config = Self::load_config(&config_path).unwrap_or_else(|_| Config {
            server: "localhost".to_string(),
            port: 25565,
            bots: vec![],
        });
        
        let logs_path = format!("{}/logs", data_dir);
        let theme_config_path = PathBuf::from(format!("{}/theme.json", data_dir));
        let theme_manager = ThemeManager::new(theme_config_path);
        let ipc_bridge = IPCBridge::new(54321);
        let _ = ipc_bridge.start_server();

        Self {
            config,
            config_path,
            data_dir,
            selected_bot: None,
            bot_processes: Arc::new(Mutex::new(vec![])),
            script_editor: String::new(),
            script_editor_backup: String::new(),
            show_script_editor: false,
            show_console: false,
            console_output: vec![],
            show_generator: false,
            generator_prompt: String::new(),
            generator_status: String::new(),
            generator_progress: 0.0,
            server_check_result: None,
            config_check_result: None,
            ai_check_result: None,
            translator: Translator::new(),
            logs_window: LogsWindow::new(logs_path),
            packet_manager: PacketManager::new(),
            discord_rich_presence: DiscordRichPresence::default(),
            ai_config: AIConfig::default(),
            show_ai_settings: false,
            show_discord_settings: false,
            show_packet_settings: false,
            theme_manager,
            show_theme_settings: false,
            ipc_bridge,
        }
    }

    fn load_config(path: &str) -> Result<Config> {
        load_toml(path)
    }

    fn save_config(&self) -> Result<()> {
        save_toml(&self.config, &self.config_path)
    }

    fn stop_bot(&self, index: usize) {
        let mut processes = self.bot_processes.lock().unwrap();
        if index < processes.len() {
            processes[index].stop();
        }
    }

    fn stop_all_bots(&mut self) {
        let mut processes = self.bot_processes.lock().unwrap();
        for process in processes.iter_mut() {
            process.stop();
        }
        processes.clear();
        for bot in &mut self.config.bots {
            bot.status = "Остановлен".to_string();
        }
    }

    fn run_bot(&self, bot: &BotConfig) -> Result<()> {
        let mut processes = self.bot_processes.lock().unwrap();
        while processes.len() <= self.config.bots.len() {
            processes.push(BotProcess::new());
        }
        
        let index = self.config.bots.iter().position(|b| b.id == bot.id).unwrap();
        processes[index].start(bot, &self.data_dir)?;
        Ok(())
    }
}

fn color_picker(ui: &mut egui::Ui, color: &mut crate::colors::Color) {
    let mut color32 = color.to_egui_color();
    ui.color_edit_button_srgba(&mut color32);
    *color = crate::colors::Color {
        r: color32.r(),
        g: color32.g(),
        b: color32.b(),
    };
}

impl BotManagerApp {
    fn delete_bot(&mut self, index: usize) {
        self.stop_bot(index);
        // Удаляем файл скрипта если он встроенный
        if !self.config.bots[index].use_external_script {
            let script_path = format!("{}/scripts/{}.js", self.data_dir, self.config.bots[index].id);
            let _ = std::fs::remove_file(script_path);
        }
        self.config.bots.remove(index);
        if self.selected_bot == Some(index) {
            self.selected_bot = None;
        } else if self.selected_bot > Some(index) {
            self.selected_bot = self.selected_bot.map(|i| i - 1);
        }
        let _ = self.save_config();
    }
}

impl eframe::App for BotManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.heading(self.translator.t("app_title"));
            ui.separator();
            
            // Переключатель языка
            ui.horizontal(|ui| {
                ui.label(self.translator.t("language") + ":");
                let current_lang = self.translator.get_language();
                if ui.selectable_label(current_lang == Language::Russian, "[RU] Russian").clicked() {
                    self.translator.set_language(Language::Russian);
                }
                if ui.selectable_label(current_lang == Language::English, "[EN] English").clicked() {
                    self.translator.set_language(Language::English);
                }
            });
            ui.separator();
        });

        egui::SidePanel::left("bot_list").default_width(250.0).show(ctx, |ui| {
            ui.label(self.translator.t("bot_list"));
            ui.separator();

            for (index, bot) in self.config.bots.iter().enumerate() {
                let selected = self.selected_bot == Some(index);
                if ui.selectable_label(selected, &bot.name).clicked() {
                    self.selected_bot = Some(index);
                }
            }

            ui.separator();
            if ui.button(format!("[+] {}", self.translator.t("add_bot"))).clicked() {
                let new_bot = BotConfig {
                    id: format!("bot{}", self.config.bots.len() + 1),
                    name: format!("NewBot{}", self.config.bots.len() + 1),
                    script_path: None,
                    script_content: get_default_script(),
                    server: self.config.server.clone(),
                    port: self.config.port,
                    account_type: AccountType::Offline,
                    username: "Player".to_string(),
                    password: String::new(),
                    enabled: true,
                    status: "Остановлен".to_string(),
                    use_external_script: false,
                    use_generated_script: false,
                    auto_login: false,
                    auto_login_password: String::new(),
                    auto_register: false,
                    auto_register_password: String::new(),
                    auto_register_twice: false,
                    connection_delay: 0,
                    minecraft_version: "1.20.4".to_string(),
                    auto_messages: vec![],
                };
                self.config.bots.push(new_bot);
                let _ = self.save_config();
            }

            ui.separator();
            if ui.button(format!("[GEN] {}", self.translator.t("generator"))).clicked() {
                self.show_generator = true;
            }

            ui.separator();
            if ui.button(format!("[STOP ALL] {}", self.translator.t("stop_all"))).clicked() {
                self.stop_all_bots();
                let _ = self.save_config();
            }

            ui.separator();
            if ui.button(format!("[AI] {}", self.translator.t("ai_settings"))).clicked() {
                self.show_ai_settings = true;
            }

            ui.separator();
            if ui.button(format!("[DISCORD] {}", self.translator.t("discord_settings"))).clicked() {
                self.show_discord_settings = true;
            }

            ui.separator();
            if ui.button(format!("[PACKETS] {}", self.translator.t("packet_settings"))).clicked() {
                self.show_packet_settings = true;
            }

            ui.separator();
            if ui.button(format!("[THEME] {}", self.translator.t("theme_settings"))).clicked() {
                self.show_theme_settings = true;
            }

            ui.separator();
            let lang_text = if self.translator.get_language() == Language::Russian {
                "EN"
            } else {
                "RU"
            };
            if ui.button(format!("[LANG] {}", lang_text)).clicked() {
                let new_lang = if self.translator.get_language() == Language::Russian {
                    Language::English
                } else {
                    Language::Russian
                };
                self.translator.set_language(new_lang);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(index) = self.selected_bot {
                let bot_id = self.config.bots[index].id.clone();
                let bot_name = self.config.bots[index].name.clone();
                let bot_script_path = self.config.bots[index].script_path.clone();
                let bot_script_content = self.config.bots[index].script_content.clone();
                let bot_server = self.config.bots[index].server.clone();
                let bot_port = self.config.bots[index].port;
                let bot_account_type = self.config.bots[index].account_type.clone();
                let bot_username = self.config.bots[index].username.clone();
                let bot_password = self.config.bots[index].password.clone();
                let bot_enabled = self.config.bots[index].enabled;
                let bot_status = self.config.bots[index].status.clone();
                let bot_use_external = self.config.bots[index].use_external_script;
                let bot_use_generated = self.config.bots[index].use_generated_script;
                let bot_auto_login = self.config.bots[index].auto_login;
                let bot_auto_messages = self.config.bots[index].auto_messages.clone();
                
                ui.group(|ui| {
                    ui.heading(format!("Настройки бота: {}", bot_name));
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("ID:");
                        ui.label(&bot_id);
                    });

                    ui.horizontal(|ui| {
                        ui.label(self.translator.t("name"));
                        if ui.text_edit_singleline(&mut self.config.bots[index].name).changed() {
                            let _ = self.save_config();
                        }
                    });

                    ui.separator();
                    ui.label(self.translator.t("connection_params"));
                    ui.horizontal(|ui| {
                        ui.label(self.translator.t("server"));
                        if ui.text_edit_singleline(&mut self.config.bots[index].server).changed() {
                            let _ = self.save_config();
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(self.translator.t("port"));
                        if ui.add(egui::DragValue::new(&mut self.config.bots[index].port).clamp_range(1..=65535)).changed() {
                            let _ = self.save_config();
                        }
                    });

                    ui.separator();
                    ui.label(self.translator.t("account_type") + ":");
                    ui.horizontal(|ui| {
                        if ui.radio(bot_account_type == AccountType::Offline, &self.translator.t("offline")).clicked() {
                            self.config.bots[index].account_type = AccountType::Offline;
                        }
                        if ui.radio(bot_account_type == AccountType::Online, &self.translator.t("online")).clicked() {
                            self.config.bots[index].account_type = AccountType::Online;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(self.translator.t("username"));
                        if ui.text_edit_singleline(&mut self.config.bots[index].username).changed() {
                            let _ = self.save_config();
                        }
                    });

                    if bot_account_type == AccountType::Online {
                        ui.horizontal(|ui| {
                            ui.label(self.translator.t("password"));
                            let mut password_display = "*".repeat(bot_password.len());
                            if ui.text_edit_singleline(&mut password_display).changed() {
                                // Пароль не редактируется в этом представлении
                            }
                        });
                    }

                    ui.separator();
                    ui.label(self.translator.t("script_type") + ":");
                    ui.horizontal(|ui| {
                        if ui.radio(bot_use_generated, &self.translator.t("auto_generation")).clicked() {
                            self.config.bots[index].use_generated_script = true;
                            self.config.bots[index].use_external_script = false;
                            // Генерируем скрипт
                            self.config.bots[index].script_content = generate_script_from_params(&self.config.bots[index]);
                        }
                        if ui.radio(!bot_use_generated && !bot_use_external, &self.translator.t("built_in_editor")).clicked() {
                            self.config.bots[index].use_generated_script = false;
                            self.config.bots[index].use_external_script = false;
                        }
                        if ui.radio(bot_use_external, &self.translator.t("external_file")).clicked() {
                            self.config.bots[index].use_external_script = true;
                            self.config.bots[index].use_generated_script = false;
                        }
                    });

                    if bot_use_generated {
                        ui.label(self.translator.t("script_auto_gen"));
                        if ui.button(format!("[REGEN] {}", self.translator.t("regenerate"))).clicked() {
                            self.config.bots[index].script_content = generate_script_from_params(&self.config.bots[index]);
                            let _ = self.save_config();
                        }
                    } else if bot_use_external {
                        ui.horizontal(|ui| {
                            ui.label(self.translator.t("script_path"));
                            if let Some(ref path) = bot_script_path {
                                ui.label(path);
                            } else {
                                ui.label(self.translator.t("not_specified"));
                            }
                        });
                        if ui.button(format!("[SELECT FILE] {}", self.translator.t("select_file"))).clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("JavaScript", &["js"])
                                .pick_file()
                            {
                                self.config.bots[index].script_path = Some(path.to_string_lossy().to_string());
                                let _ = self.save_config();
                            }
                        }
                    } else {
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.label(self.translator.t("settings") + ":");
                            if ui.button(self.translator.t("open_editor")).clicked() {
                                self.show_script_editor = true;
                                self.script_editor = bot_script_content.clone();
                                self.script_editor_backup = bot_script_content.clone();
                            }
                        });
                    }

                    ui.separator();
                    ui.label(self.translator.t("automation") + ":");
                    ui.checkbox(&mut self.config.bots[index].auto_login, &self.translator.t("auto_login"));
                    
                    // Показываем поле пароля если включен авто-логин
                    if self.config.bots[index].auto_login {
                        ui.horizontal(|ui| {
                            ui.label(self.translator.t("auto_password"));
                            ui.text_edit_singleline(&mut self.config.bots[index].auto_login_password);
                        });
                    }

                    ui.checkbox(&mut self.config.bots[index].auto_register, &self.translator.t("auto_register"));
                    
                    // Показываем поле пароля если включена авто-регистрация
                    if self.config.bots[index].auto_register {
                        ui.horizontal(|ui| {
                            ui.label(self.translator.t("auto_register_password"));
                            ui.text_edit_singleline(&mut self.config.bots[index].auto_register_password);
                        });
                        ui.checkbox(&mut self.config.bots[index].auto_register_twice, &self.translator.t("auto_register_twice"));
                    }

                    ui.horizontal(|ui| {
                        ui.label(self.translator.t("connection_delay"));
                        ui.add(egui::DragValue::new(&mut self.config.bots[index].connection_delay).clamp_range(0..=60));
                    });

                    ui.horizontal(|ui| {
                        ui.label(self.translator.t("minecraft_version"));
                        ui.text_edit_singleline(&mut self.config.bots[index].minecraft_version);
                    });
                    
                    ui.label(self.translator.t("auto_messages"));
                    for i in 0..bot_auto_messages.len() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}:", i + 1));
                            if ui.text_edit_singleline(&mut self.config.bots[index].auto_messages[i]).changed() {
                                let _ = self.save_config();
                            }
                            if ui.button("[DEL]").clicked() {
                                self.config.bots[index].auto_messages.remove(i);
                                let _ = self.save_config();
                            }
                        });
                    }
                    if ui.button(format!("[+] {}", self.translator.t("add_message"))).clicked() {
                        self.config.bots[index].auto_messages.push(String::new());
                        let _ = self.save_config();
                    }

                    ui.separator();
                    ui.label(self.translator.t("enabled_status").replace("{}", &bot_enabled.to_string()));
                    
                    ui.separator();
                    let status_text = if self.translator.get_language() == Language::Russian {
                        if bot_status == "Запущен" || bot_status == "Running" {
                            "Запущен".to_string()
                        } else if bot_status == "Остановлен" || bot_status == "Stopped" {
                            "Остановлен".to_string()
                        } else {
                            bot_status.clone()
                        }
                    } else {
                        if bot_status == "Запущен" || bot_status == "Running" {
                            "Running".to_string()
                        } else if bot_status == "Остановлен" || bot_status == "Stopped" {
                            "Stopped".to_string()
                        } else {
                            bot_status.clone()
                        }
                    };
                    ui.label(format!("{}: {}", self.translator.t("status"), status_text));

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button(format!("[CONSOLE] {}", self.translator.t("console"))).clicked() {
                            self.show_console = true;
                            let processes = self.bot_processes.lock().unwrap();
                            if index < processes.len() {
                                self.console_output = processes[index].get_output();
                            }
                        }
                        
                        if ui.button(format!("[LOGS] {}", self.translator.t("logs"))).clicked() {
                            if let Some(index) = self.selected_bot {
                                let bot_id = self.config.bots[index].id.clone();
                                self.logs_window.selected_bot = Some(bot_id);
                                self.logs_window.open();
                            }
                        }
                    });

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button(format!("▶ {}", self.translator.t("start"))).clicked() {
                            let bot = &self.config.bots[index];
                            match self.run_bot(bot) {
                                Ok(_) => {
                                    self.config.bots[index].status = if self.translator.get_language() == Language::Russian {
                                        "Запущен".to_string()
                                    } else {
                                        "Running".to_string()
                                    };
                                }
                                Err(e) => {
                                    self.config.bots[index].status = format!("Error: {}", e);
                                }
                            }
                        }

                        if ui.button(self.translator.t("stop")).clicked() {
                            self.stop_bot(index);
                            self.config.bots[index].status = if self.translator.get_language() == Language::Russian {
                                "Остановлен".to_string()
                            } else {
                                "Stopped".to_string()
                            };
                        }

                        if ui.button(format!("[SAVE] {}", self.translator.t("save"))).clicked() {
                            let _ = self.save_config();
                        }

                        if ui.button(format!("[DELETE] {}", self.translator.t("delete"))).clicked() {
                            self.delete_bot(index);
                        }
                    });
                });
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label(self.translator.t("select_bot"));
                });
            }
        });

        // Окно редактора скриптов
        if self.show_script_editor {
            egui::Window::new("Редактор скрипта")
                .collapsible(false)
                .resizable(true)
                .default_size([600.0, 500.0])
                .show(ctx, |ui| {
                    ui.label(self.translator.t("edit_script"));
                    ui.separator();
                    
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_sized(
                            [ui.available_width(), 400.0],
                            egui::TextEdit::multiline(&mut self.script_editor)
                                .code_editor()
                                .desired_rows(25)
                        );
                    });
                    
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button(format!("[SAVE] {}", self.translator.t("save"))).clicked() {
                            if let Some(index) = self.selected_bot {
                                self.config.bots[index].script_content = self.script_editor.clone();
                                let _ = self.save_config();
                            }
                            self.show_script_editor = false;
                        }
                        
                        if ui.button(format!("[X] {}", self.translator.t("cancel"))).clicked() {
                            self.script_editor = self.script_editor_backup.clone();
                            self.show_script_editor = false;
                        }
                        
                        if ui.button(format!("[DEL SCRIPT] {}", self.translator.t("delete_script"))).clicked() {
                            if let Some(index) = self.selected_bot {
                                self.config.bots[index].script_content = String::new();
                                let _ = self.save_config();
                            }
                            self.show_script_editor = false;
                        }
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(format!("[X] {}", self.translator.t("close"))).clicked() {
                                self.show_script_editor = false;
                            }
                        });
                    });
                });
        }

        // Окно консоли бота
        if self.show_console {
            egui::Window::new(self.translator.t("console_output"))
                .collapsible(false)
                .resizable(true)
                .default_size([800.0, 500.0])
                .show(ctx, |ui| {
                    ui.label(self.translator.t("console_output"));
                    ui.separator();
                    
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for line in &self.console_output {
                            ui.label(line);
                        }
                    });
                    
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button(format!("[REFRESH] {}", self.translator.t("update"))).clicked() {
                            if let Some(index) = self.selected_bot {
                                let processes = self.bot_processes.lock().unwrap();
                                if index < processes.len() {
                                    self.console_output = processes[index].get_output();
                                }
                            }
                        }
                        
                        if ui.button(format!("[CLEAR] {}", self.translator.t("clear"))).clicked() {
                            self.console_output.clear();
                        }
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(format!("[X] {}", self.translator.t("close"))).clicked() {
                                self.show_console = false;
                            }
                        });
                    });
                });
        }

        // Окно генератора ботов
        if self.show_generator {
            egui::Window::new(self.translator.t("generator"))
                .collapsible(false)
                .resizable(true)
                .default_size([600.0, 500.0])
                .show(ctx, |ui| {
                    ui.label(self.translator.t("prompt_placeholder"));
                    ui.separator();
                    
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_sized(
                            [ui.available_width(), 150.0],
                            egui::TextEdit::multiline(&mut self.generator_prompt)
                                .hint_text(if self.translator.get_language() == Language::Russian {
                                    "Введите описание бота..."
                                } else {
                                    "Enter bot description..."
                                })
                        );
                    });
                    
                    ui.separator();
                    
                    if !self.generator_status.is_empty() {
                        ui.label(&self.generator_status);
                        ui.add(egui::ProgressBar::new(self.generator_progress).show_percentage());
                    }
                    
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button(format!("[GENERATE] {}", self.translator.t("generate"))).clicked() {
                            self.generator_status = if self.translator.get_language() == Language::Russian {
                                "Анализ промпта...".to_string()
                            } else {
                                "Analyzing prompt...".to_string()
                            };
                            self.generator_progress = 0.3;
                            
                            match parse_prompt(&self.generator_prompt) {
                                Ok(params) => {
                                    self.generator_status = if self.translator.get_language() == Language::Russian {
                                        "Проверка сервера...".to_string()
                                    } else {
                                        "Checking server...".to_string()
                                    };
                                    self.generator_progress = 0.5;
                                    
                                    let result = check_server(&params.server, params.port);
                                    self.server_check_result = Some(result.clone());
                                    
                                    if result.reachable {
                                        self.generator_status = if self.translator.get_language() == Language::Russian {
                                            format!("Сервер доступен! Пинг: {}мс", result.latency_ms.unwrap_or(0))
                                        } else {
                                            format!("Server available! Ping: {}ms", result.latency_ms.unwrap_or(0))
                                        };
                                        self.generator_progress = 0.7;
                                        
                                        let new_bot = build_bot_from_prompt(params, self.config.bots.len());
                                        let check_result = validate_bot_config(&new_bot);
                                        self.config_check_result = Some(check_result.clone());
                                        
                                        if check_result.valid {
                                            self.config.bots.push(new_bot);
                                            let _ = self.save_config();
                                            self.generator_status = if self.translator.get_language() == Language::Russian {
                                                "Бот успешно создан!".to_string()
                                            } else {
                                                "Bot created successfully!".to_string()
                                            };
                                            self.generator_progress = 1.0;
                                        } else {
                                            self.generator_status = if self.translator.get_language() == Language::Russian {
                                                format!("Ошибки конфигурации: {:?}", check_result.errors)
                                            } else {
                                                format!("Configuration errors: {:?}", check_result.errors)
                                            };
                                            self.generator_progress = 0.0;
                                        }
                                    } else {
                                        self.generator_status = if self.translator.get_language() == Language::Russian {
                                            format!("Сервер недоступен: {}", result.error.unwrap_or_default())
                                        } else {
                                            format!("Server unavailable: {}", result.error.unwrap_or_default())
                                        };
                                        self.generator_progress = 0.0;
                                    }
                                }
                                Err(e) => {
                                    self.generator_status = if self.translator.get_language() == Language::Russian {
                                        format!("Ошибка парсинга: {}", e)
                                    } else {
                                        format!("Parsing error: {}", e)
                                    };
                                    self.generator_progress = 0.0;
                                }
                            }
                        }
                        
                        if ui.button(format!("[X] {}", self.translator.t("cancel"))).clicked() {
                            self.show_generator = false;
                            self.generator_prompt.clear();
                            self.generator_status.clear();
                            self.generator_progress = 0.0;
                        }
                    });
                    
                    // Показываем результаты проверок
                    if let Some(ref check) = self.server_check_result {
                        ui.separator();
                        ui.label(self.translator.t("server_check_result"));
                        if check.reachable {
                            ui.label(self.translator.t("server_available").replace("{}", &check.latency_ms.unwrap_or(0).to_string()));
                        } else {
                            ui.label(self.translator.t("server_unavailable").replace("{}", check.error.as_ref().unwrap_or(&"Unknown error".to_string())));
                        }
                    }
                    
                    if let Some(ref check) = self.config_check_result {
                        ui.separator();
                        ui.label(self.translator.t("config_check_result"));
                        if check.valid {
                            ui.label(self.translator.t("config_valid"));
                        } else {
                            for error in &check.errors {
                                ui.label(format!("[X] {}", error));
                            }
                        }
                        for warning in &check.warnings {
                            ui.label(format!("[WARN] {}", warning));
                        }
                    }
                });
        }

        // Окно логов
        let bot_ids: Vec<String> = self.config.bots.iter().map(|b| b.id.clone()).collect();
        self.logs_window.show(ctx, &bot_ids);

        // Окно настроек ИИ
        if self.show_ai_settings {
            egui::Window::new(self.translator.t("ai_settings"))
                .collapsible(false)
                .resizable(true)
                .default_size([600.0, 500.0])
                .show(ctx, |ui| {
                    ui.label(self.translator.t("api_key"));
                    ui.text_edit_singleline(&mut self.ai_config.api_key);
                    
                    ui.label(self.translator.t("ai_model"));
                    ui.text_edit_singleline(&mut self.ai_config.model);
                    
                    ui.label(self.translator.t("ai_provider"));
                    ui.text_edit_singleline(&mut self.ai_config.provider);
                    
                    ui.horizontal(|ui| {
                        ui.label(self.translator.t("max_tokens"));
                        ui.add(egui::DragValue::new(&mut self.ai_config.max_tokens).clamp_range(1..=4000));
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label(self.translator.t("temperature"));
                        ui.add(egui::DragValue::new(&mut self.ai_config.temperature).clamp_range(0.0..=2.0).speed(0.1));
                    });
                    
                    ui.label(self.translator.t("system_prompt"));
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_sized(
                            [ui.available_width(), 100.0],
                            egui::TextEdit::multiline(&mut self.ai_config.system_prompt)
                        );
                    });
                    
                    ui.separator();
                    ui.checkbox(&mut self.ai_config.enabled, &self.translator.t("enable_ai"));
                    
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button(format!("[CHECK] {}", self.translator.t("check_ai"))).clicked() {
                            let result = check_ai_config(&self.ai_config.api_key, &self.ai_config.model, &self.ai_config.provider);
                            self.ai_check_result = Some(result.unwrap_or_default());
                        }
                        
                        if ui.button("[X] Close").clicked() {
                            self.show_ai_settings = false;
                        }
                    });
                    
                    if let Some(ref check) = self.ai_check_result {
                        ui.separator();
                        ui.label(self.translator.t("ai_check_result"));
                        if check.valid {
                            ui.label("[OK] AI configuration is valid");
                        } else {
                            for error in &check.errors {
                                ui.label(format!("[X] {}", error));
                            }
                        }
                        for warning in &check.warnings {
                            ui.label(format!("[WARN] {}", warning));
                        }
                    }
                });
        }

        // Окно настроек Discord
        if self.show_discord_settings {
            egui::Window::new(self.translator.t("discord_settings"))
                .collapsible(false)
                .resizable(true)
                .default_size([500.0, 400.0])
                .show(ctx, |ui| {
                    let config = self.discord_rich_presence.get_config_mut();
                    ui.label("Client ID:");
                    ui.text_edit_singleline(&mut config.client_id);
                    
                    ui.label("Public Key:");
                    ui.text_edit_singleline(&mut config.public_key);
                    
                    ui.separator();
                    ui.checkbox(&mut config.enabled, &self.translator.t("discord_enabled"));
                    
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("[CONNECT] Connect").clicked() {
                            let _ = self.discord_rich_presence.connect();
                        }
                        
                        if ui.button("[DISCONNECT] Disconnect").clicked() {
                            self.discord_rich_presence.disconnect();
                        }
                        
                        if ui.button("[X] Close").clicked() {
                            self.show_discord_settings = false;
                        }
                    });
                    
                    ui.separator();
                    if self.discord_rich_presence.is_connected() {
                        ui.label("[OK] Connected to Discord");
                    } else {
                        ui.label("[X] Not connected to Discord");
                    }
                });
        }

        // Окно настроек темы
        if self.show_theme_settings {
            egui::Window::new(self.translator.t("theme_settings"))
                .collapsible(false)
                .resizable(true)
                .default_size([400.0, 500.0])
                .show(ctx, |ui| {
                    let mut palette = self.theme_manager.get_palette();
                    
                    ui.label(self.translator.t("background_color"));
                    color_picker(ui, &mut palette.background);
                    
                    ui.label(self.translator.t("panel_color"));
                    color_picker(ui, &mut palette.panel);
                    
                    ui.label(self.translator.t("text_color"));
                    color_picker(ui, &mut palette.text);
                    
                    ui.label(self.translator.t("accent_color"));
                    color_picker(ui, &mut palette.accent);
                    
                    ui.label(self.translator.t("button_color"));
                    color_picker(ui, &mut palette.button);
                    
                    ui.label(self.translator.t("border_color"));
                    color_picker(ui, &mut palette.border);
                    
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("[APPLY] Apply").clicked() {
                            self.theme_manager.set_palette(palette.clone());
                            let _ = self.theme_manager.save();
                            ctx.set_style(palette.to_egui_style());
                        }
                        
                        if ui.button("[RESET] Reset").clicked() {
                            let default_palette = ColorPalette::default();
                            self.theme_manager.set_palette(default_palette.clone());
                            let _ = self.theme_manager.save();
                            ctx.set_style(default_palette.to_egui_style());
                        }
                        
                        if ui.button("[X] Close").clicked() {
                            self.show_theme_settings = false;
                        }
                    });
                });
        }

        // Окно настроек пакетов
        if self.show_packet_settings {
            egui::Window::new(self.translator.t("packet_settings"))
                .collapsible(false)
                .resizable(true)
                .default_size([500.0, 400.0])
                .show(ctx, |ui| {
                    let mut rate_limit = true;
                    ui.checkbox(&mut rate_limit, &self.translator.t("rate_limit"));
                    
                    ui.horizontal(|ui| {
                        ui.label(self.translator.t("min_interval"));
                        let mut interval = 50;
                        ui.add(egui::DragValue::new(&mut interval).clamp_range(10..=500));
                    });
                    
                    ui.separator();
                    ui.label("Packet Statistics:");
                    let stats = self.packet_manager.get_stats();
                    ui.label(format!("Total sent: {}", stats.total_sent));
                    ui.label(format!("Total received: {}", stats.total_received));
                    
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("[RESET] Reset Stats").clicked() {
                            self.packet_manager.reset_stats();
                        }
                        
                        if ui.button("[X] Close").clicked() {
                            self.show_packet_settings = false;
                        }
                    });
                });
        }

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

fn main() -> eframe::Result<()> {
    let icon_data = eframe::icon_data::from_png_bytes(&include_bytes!("../../../assets/Minecraft.png")[..])
        .unwrap_or_default();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([700.0, 500.0])
            .with_icon(icon_data),
        ..Default::default()
    };

    eframe::run_native(
        "Js Simple Botzz",
        options,
        Box::new(|cc| Box::new(BotManagerApp::new(cc))),
    )
}
