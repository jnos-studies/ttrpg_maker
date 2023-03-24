use eframe::egui;
use regex::Regex;
use entities;
use std::env;
use std::fs;
use store_rpg::database_setup;

const NAVIGATION_SELECTION_SIZE: f32 = 20.0;

#[derive(Default)]
pub struct TTRPGMaker {
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    load_ttrpg: bool,
    database_path: String,
    file_save: String
}

impl eframe::App for TTRPGMaker {
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.visuals_mut().selection.bg_fill = egui::Color32::DARK_GRAY;
            ui.visuals_mut().selection.stroke.color = egui::Color32::BLACK;
            let load_menu_text = set_text_widget_size("options".to_string(), NAVIGATION_SELECTION_SIZE)
                .color(egui::Color32::WHITE);
            ui.menu_button(load_menu_text, |ui| {
                if ui.button("load / create ttrpg").clicked() {
                   self.load_ttrpg = true; 
                }
            });
        });

        if self.load_ttrpg {
            egui::Window::new("saved ttrpgs")
                .collapsible(true)
                .resizable(true)
                .show(ctx, |ui| {
                self.database_path = match env::var("DATABASE_PATH") {
                    Ok(path) => path,
                    Err(_) => String::from("No path"),
                };
                if ui.button("X").clicked()
                {
                    self.load_ttrpg = false;
                }
                if self.database_path == "No path" && fs::read_dir("saves/").unwrap().count() == 0 {
                    let check_exists_file_name: bool = format!("saves/{}.db", self.file_save).eq(&self.database_path);
                    ui.text_edit_singleline(&mut self.file_save);
                    if ui.button("Create!").clicked() && !check_exists_file_name {
                        if !self.file_save.contains(char::is_whitespace) && self.file_save.len() > 0 // if the file save does not contain whitespaces or nothing
                        {
                            self.database_path = format!("saves/{}.db", self.file_save);
                            env::set_var("DATABASE_PATH", &self.database_path.as_str());
                            database_setup(&self.database_path.as_str()); // need to add error handling to this, return a Result to unwrap
                        }
                        //LOAD DATABASE AND CLOSE WINDOW
                        self.load_ttrpg = false;   
                    }
                } else {
                    //Load previously created ttrpg databases
                    let paths = fs::read_dir("saves/").unwrap();
                    for path in paths
                    {
                        let p = path.unwrap().path().display().to_string();
                        
                        if ui.add(egui::Button::new(&p)).clicked()
                        {
                            self.file_save = p.as_str().clone().to_string();
                            env::set_var("DATABASE_PATH", &self.file_save.as_str());
                            
                            //LOAD THE DATABASE AND CLOSE WINDOW
                        }
                    }
                    ui.horizontal(|ui| {
                        let check_exists_file_name: bool = self.file_save.eq(&self.database_path);
                        ui.text_edit_singleline(&mut self.file_save);
                        if ui.button("Create!").clicked() && !check_exists_file_name
                        {
                            if !self.file_save.contains(char::is_whitespace) && self.file_save.len() > 0 // if the file save does not contain whitespaces or nothing
                            {
                                self.database_path = format!("saves/{}.db", self.file_save);
                                env::set_var("DATABASE_PATH", &self.database_path.as_str());
                                database_setup(&self.database_path.as_str());
                            }
                            //LOAD DATABASE THEN CLOSE WINDOW
                       }
                    });
                }
            });
        }


        if self.show_confirmation_dialog {
            // Show confirmation dialog:
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_confirmation_dialog = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.allowed_to_close = true;
                            frame.close();
                        }
                    });
                });
        }
    }
}

pub fn start_app_main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "TTRPG Maker",
        options,
        Box::new(|_cc| Box::new(TTRPGMaker::default())),
    )
}

fn set_text_widget_size(text: String, size: f32) -> egui::WidgetText {
    let text = egui::RichText::new(text).size(size);
    egui::WidgetText::from(text)
}

fn escape_sql(input: &str) -> String {
    let re = Regex::new(r#"([\\'"])"#).unwrap();
    let escaped = re.replace_all(input, "");
    escaped.to_string()

}
