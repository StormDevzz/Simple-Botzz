use eframe::egui;
use std::sync::{Arc, Mutex};

/// Окно консоли бота
pub struct ConsoleWindow {
    pub visible: bool,
    pub output_lines: Arc<Mutex<Vec<String>>>,
}

impl ConsoleWindow {
    pub fn new() -> Self {
        Self {
            visible: false,
            output_lines: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn open(&mut self, lines: Vec<String>) {
        *self.output_lines.lock().unwrap() = lines;
        self.visible = true;
    }

    pub fn close(&mut self) {
        self.visible = false;
    }

    pub fn update_output(&mut self, lines: Vec<String>) {
        *self.output_lines.lock().unwrap() = lines;
    }

    pub fn show(&mut self, ctx: &egui::Context) -> bool {
        let mut should_close = false;
        
        if self.visible {
            egui::Window::new("Bot Console")
                .collapsible(false)
                .resizable(true)
                .default_size([800.0, 500.0])
                .show(ctx, |ui| {
                    ui.label("Console output:");
                    ui.separator();
                    
                    let lines = self.output_lines.lock().unwrap();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for line in lines.iter() {
                            ui.label(line);
                        }
                    });
                    
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("[REFRESH] Refresh").clicked() {
                            // Refresh handled by parent
                        }
                        
                        if ui.button("[CLEAR] Clear").clicked() {
                            self.output_lines.lock().unwrap().clear();
                        }
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("[X] Close").clicked() {
                                should_close = true;
                            }
                        });
                    });
                });
        }
        
        if should_close {
            self.visible = false;
        }
        
        self.visible
    }
}
