use eframe::egui;

/// Окно редактора скриптов
pub struct EditorWindow {
    pub visible: bool,
    pub content: String,
    pub backup: String,
}

impl EditorWindow {
    pub fn new() -> Self {
        Self {
            visible: false,
            content: String::new(),
            backup: String::new(),
        }
    }

    pub fn open(&mut self, content: String) {
        self.content = content.clone();
        self.backup = content;
        self.visible = true;
    }

    pub fn close(&mut self) {
        self.visible = false;
    }

    pub fn cancel(&mut self) {
        self.content = self.backup.clone();
        self.visible = false;
    }

    pub fn clear(&mut self) {
        self.content.clear();
    }

    pub fn show(&mut self, ctx: &egui::Context, on_save: impl FnOnce(&str)) -> bool {
        let mut should_close = false;
        
        if self.visible {
            egui::Window::new("Script Editor")
                .collapsible(false)
                .resizable(true)
                .default_size([800.0, 600.0])
                .show(ctx, |ui| {
                    ui.label("Edit bot script:");
                    ui.separator();
                    
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_sized(
                            [ui.available_width(), 400.0],
                            egui::TextEdit::multiline(&mut self.content)
                                .code_editor()
                                .desired_rows(25)
                        );
                    });
                    
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("[SAVE] Save").clicked() {
                            on_save(&self.content);
                            should_close = true;
                        }
                        
                        if ui.button("[CANCEL] Cancel").clicked() {
                            self.cancel();
                            should_close = true;
                        }
                        
                        if ui.button("[CLEAR] Clear Script").clicked() {
                            self.clear();
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
