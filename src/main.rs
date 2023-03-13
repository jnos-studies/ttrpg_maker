#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 300.0)),
        ..Default::default()
    };
    eframe::run_native(
        "TTRPG Maker",
        options,
        Box::new(|_cc| Box::new(Menu::default())),
    )
}

struct Menu {
    name: String,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
        }
    }
}

impl eframe::App for Menu {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.button("Load TTRPG");
            let new_session_label = ui.label("New TTRPG: ");
            ui.text_edit_singleline(&mut self.name)
                .labelled_by(new_session_label.id)
        });
    }
}
