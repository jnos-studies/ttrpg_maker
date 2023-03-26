use eframe::egui;
use regex::Regex;
use entities::*;
use narratives::*;
use std::env;
use std::fs;
use store_rpg::{database_setup, load_ttrpg};

const NAVIGATION_SELECTION_SIZE: f32 = 20.0;

#[derive(Default)]
pub struct TTRPGMaker
{
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    load_ttrpg: std::cell::Cell<bool>,
    database_path: String,
    file_save: String
}

impl eframe::App for TTRPGMaker
{
    fn on_close_event(&mut self) -> bool
    {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame)
    {
        egui::TopBottomPanel::top("menu").show(ctx, |ui|
        {
            ui.visuals_mut().selection.bg_fill = egui::Color32::DARK_GRAY;
            ui.visuals_mut().selection.stroke.color = egui::Color32::BLACK;
            let load_menu_text = set_text_widget_size("options".to_string(), NAVIGATION_SELECTION_SIZE)
                .color(egui::Color32::WHITE);
            ui.menu_button(load_menu_text, |ui|
            {
                if ui.button("load / create ttrpg").clicked()
                {
                   self.load_ttrpg.set(true); 
                }
            });
        });

        if self.load_ttrpg.get()
        {
            egui::Window::new("pick a database").open(&mut self.load_ttrpg.get_mut())
                .collapsible(false)
                .resizable(false)
                .id(egui::Id::new("create_menu"))
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
                .show(ctx, |ui| {
                self.database_path = match env::var("DATABASE_PATH")
                {
                    Ok(path) => path,
                    Err(_) => String::from("No path"),
                };
                if self.database_path == "No path" && fs::read_dir("saves/").unwrap().count() == 0
                {
                    let check_exists_file_name: bool = format!("saves/{}.db", self.file_save).eq(&self.database_path);
                    ui.text_edit_singleline(&mut self.file_save);
                    if ui.button("Create!").clicked() && !check_exists_file_name
                    {
                        if !self.file_save.contains(char::is_whitespace) &&
                        self.file_save.len() > 0 && // if the file save does not contain whitespaces or nothing
                        check_non_alphanumertic(&self.file_save.as_str())
                        {
                            self.database_path = format!("saves/{}.db", self.file_save);
                            env::set_var("DATABASE_PATH", &self.database_path.as_str());
                            database_setup(&self.database_path.as_str()); // need to add error handling to this, return a Result to unwrap
                        }
                        //LOAD DATABASE AND CLOSE WINDOW
                        //in store_rpg, make function load_ttrpg()
                    }
                } else {
                    //Load previously created ttrpg databases
                    let paths = fs::read_dir("saves/").unwrap();
                    for path in paths
                    {
                        let p = path.unwrap().path().display().to_string();
                        let path_button = ui.add_sized((ui.available_width(), 10.0), egui::Button::new(&p));
                        path_button.context_menu(|ui|
                        {
                           if ui.small_button("Delete").clicked()
                           {
                                fs::remove_file(&p).unwrap();
                                ctx.request_repaint(); //repaint the ui after deleting the file
                           }
                           if ui.small_button("Load").clicked()
                           {
                                self.file_save = p.clone();
                                env::set_var("DATABASE_PATH", &self.file_save.as_str());
                                self.file_save = "".to_string(); //empty the single line
                                //LOAD THE DATABASE AND CLOSE WINDOW
                                //in store_rpg, make function load_ttrpg()
                           }
                        });
                    }
                    ui.horizontal(|ui|
                    {
                        let check_exists_file_name: bool = format!("saves/{}.db", self.file_save).eq(&self.database_path);
                        ui.text_edit_singleline(&mut self.file_save);
                        if ui.button("Create!").clicked() && !check_exists_file_name
                        {
                            if !self.file_save.contains(char::is_whitespace) &&
                            self.file_save.len() > 0 &&
                            check_non_alphanumertic(&self.file_save.as_str())
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

        if self.show_confirmation_dialog
        {
            // Show confirmation dialog:
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui|
                {
                    ui.horizontal(|ui|
                    {
                        if ui.button("Cancel").clicked()
                        {
                            self.show_confirmation_dialog = false;
                        }

                        if ui.button("Yes!").clicked()
                        {
                            self.allowed_to_close = true;
                            frame.close();
                        }
                    });
                });
        }
    }
}

pub fn start_app_main() -> Result<(), eframe::Error>
{
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "TTRPG Maker",
        options,
        Box::new(|_cc| Box::new(TTRPGMaker::default())),
    )
}

fn set_text_widget_size(text: String, size: f32) -> egui::WidgetText
{
    let text = egui::RichText::new(text).size(size);
    egui::WidgetText::from(text)
}

fn escape_sql(input: &str) -> String
{
    let re = Regex::new(r#"([\\'"])"#).unwrap();
    let escaped = re.replace_all(input, "");
    escaped.to_string()

}

fn check_non_alphanumertic (input:&str) -> bool
{
    for c in input.chars()
    {
        if !c.is_alphanumeric()
        {
            return false;
        }
    }
    return true
}
    
