use eframe::egui;
use std::collections::HashMap;
use std::path::PathBuf;

/// Окно просмотра логов ботов
pub struct LogsWindow {
    pub visible: bool,
    pub selected_bot: Option<String>,
    pub logs: HashMap<String, Vec<String>>,
    pub log_dir: String,
}

impl LogsWindow {
    pub fn new(log_dir: String) -> Self {
        Self {
            visible: false,
            selected_bot: None,
            logs: HashMap::new(),
            log_dir,
        }
    }

    pub fn open(&mut self) {
        self.visible = true;
        self.load_all_logs();
    }

    pub fn close(&mut self) {
        self.visible = false;
    }

    pub fn add_log(&mut self, bot_id: &str, message: &str) {
        let entry = self.logs.entry(bot_id.to_string()).or_insert_with(Vec::new);
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        entry.push(format!("[{}] {}", timestamp, message));
        
        // Limit log size
        if entry.len() > 1000 {
            entry.remove(0);
        }
    }

    pub fn get_bot_logs(&self, bot_id: &str) -> Vec<String> {
        self.logs.get(bot_id).cloned().unwrap_or_default()
    }

    pub fn load_all_logs(&mut self) {
        // Load from files if they exist
        let log_path = PathBuf::from(&self.log_dir);
        if let Ok(entries) = std::fs::read_dir(log_path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".log") {
                        let bot_id = name.trim_end_matches(".log");
                        if let Ok(content) = std::fs::read_to_string(entry.path()) {
                            let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
                            self.logs.insert(bot_id.to_string(), lines);
                        }
                    }
                }
            }
        }
    }

    pub fn save_logs(&self, bot_id: &str) {
        if let Some(lines) = self.logs.get(bot_id) {
            let log_path = PathBuf::from(&self.log_dir).join(format!("{}.log", bot_id));
            let content = lines.join("\n");
            let _ = std::fs::write(log_path, content);
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, bot_ids: &[String]) -> bool {
        let mut should_close = false;
        
        if self.visible {
            egui::Window::new("Bot Logs")
                .collapsible(false)
                .resizable(true)
                .default_size([900.0, 600.0])
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Select bot:");
                        for bot_id in bot_ids {
                            let selected = self.selected_bot.as_ref() == Some(bot_id);
                            if ui.selectable_label(selected, bot_id).clicked() {
                                self.selected_bot = Some(bot_id.clone());
                            }
                        }
                    });
                    
                    ui.separator();
                    
                    if let Some(ref bot_id) = self.selected_bot {
                        let bot_id = bot_id.clone();
                        ui.label(format!("Logs for bot: {}", bot_id));
                        ui.separator();
                        
                        let logs = self.get_bot_logs(&bot_id);
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for line in logs.iter().rev().take(100) {
                                ui.label(line);
                            }
                        });
                        
                        ui.separator();
                        ui.horizontal(|ui| {
                            if ui.button("[REFRESH] Refresh").clicked() {
                                self.load_all_logs();
                            }
                            
                            if ui.button("[CLEAR] Clear Logs").clicked() {
                                self.logs.insert(bot_id.clone(), vec![]);
                                self.save_logs(&bot_id);
                            }
                            
                            if ui.button("[EXPORT] Export to File").clicked() {
                                self.save_logs(&bot_id);
                            }
                        });
                    } else {
                        ui.label("Select a bot to view logs");
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("[X] Close").clicked() {
                            should_close = true;
                        }
                    });
                });
        }
        
        if should_close {
            self.visible = false;
        }
        
        self.visible
    }
}
