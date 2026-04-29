use eframe::egui;

/// Окно генератора ботов
pub struct GeneratorWindow {
    pub visible: bool,
    pub prompt: String,
    pub status: String,
    pub progress: f32,
}

impl GeneratorWindow {
    pub fn new() -> Self {
        Self {
            visible: false,
            prompt: String::new(),
            status: String::new(),
            progress: 0.0,
        }
    }

    pub fn open(&mut self) {
        self.visible = true;
        self.prompt.clear();
        self.status.clear();
        self.progress = 0.0;
    }

    pub fn close(&mut self) {
        self.visible = false;
    }

    pub fn set_status(&mut self, status: &str, progress: f32) {
        self.status = status.to_string();
        self.progress = progress;
    }

    pub fn show(&mut self, ctx: &egui::Context, on_generate: impl FnOnce(&str)) -> bool {
        let mut should_close = false;
        let mut should_generate = false;
        
        if self.visible {
            egui::Window::new("Bot Generator")
                .collapsible(false)
                .resizable(true)
                .default_size([600.0, 500.0])
                .show(ctx, |ui| {
                    ui.label("Describe the bot (example: 'Create bot with username Steve for server 192.168.1.1:25565, offline mode, auto-login'):");
                    ui.separator();
                    
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_sized(
                            [ui.available_width(), 150.0],
                            egui::TextEdit::multiline(&mut self.prompt)
                                .hint_text("Enter bot description...")
                        );
                    });
                    
                    ui.separator();
                    
                    if !self.status.is_empty() {
                        ui.label(&self.status);
                        ui.add(egui::ProgressBar::new(self.progress).show_percentage());
                    }
                    
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("[GENERATE] Generate").clicked() {
                            should_generate = true;
                        }
                        
                        if ui.button("[CANCEL] Cancel").clicked() {
                            should_close = true;
                        }
                    });
                });
        }
        
        if should_generate {
            on_generate(&self.prompt);
        }
        
        if should_close {
            self.visible = false;
        }
        
        self.visible
    }
}
