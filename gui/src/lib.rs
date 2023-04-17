use eframe::egui;
use eframe::epaint::image;
use regex::Regex;
use entities::*;
use narratives::*;
use std::env;
use std::fs;
use std::io::Read;
use store_rpg::*;
use libtext::*;
use sqlite;
use std::sync::{Arc, Mutex};

pub struct TTRPGMaker
{
    load_database: std::cell::Cell<bool>,
    load_elements: std::cell::Cell<bool>,
    load_creation: std::cell::Cell<bool>,
    conn: sqlite::Connection,
    databases: Vec<String>,
    selected: Option<String>,
    elements: std::cell::Cell<bool>,
    loaded_ttrpg: Returned_TTRPG
}

impl Default for TTRPGMaker
{
    fn default() -> Self
    {
        // Set the load database to true and the other window bools to false
        let load_database = std::cell::Cell::new(true);
        let load_elements = std::cell::Cell::new(false);
        let load_creation = std::cell::Cell::new(false);
        // Create a connection to a SQLite database in memory
        let conn = sqlite::open(":memory:").unwrap();
        // Get the list of available databases
        let databases = std::fs::read_dir("saves/")
            .unwrap()
            .into_iter()
            .map(|db| {db.unwrap().file_name().into_string().unwrap()})
            .collect();
       

        // Initialize the selected database to None
        let selected = None;
        let elements = std::cell::Cell::new(false);
        // This is a dummy value to be overwritten later
        let loaded_ttrpg = store_rpg::Returned_TTRPG
        {
            name: "No ttrpg selected".to_string(),
            id: 0,
            stories: Vec::new(),
            attributes: Vec::new(),
            skills: Vec::new(),
            counters: Vec::new(),
            tables: Vec::new()
        };
        Self
        {
            load_database,
            load_elements,
            load_creation,
            conn,
            databases,
            selected,
            elements,
            loaded_ttrpg
        }
    }
}

impl eframe::App for TTRPGMaker {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Create a UI with a label and a combo box
        frame.set_centered();
        egui::TopBottomPanel::top("tabs")
            .show_separator_line(false)
            .show(ctx, |ui| {
                // TODO: Need to define a style for the tabs
                let mut tabs = ui.child_ui(
                    ui.available_rect_before_wrap(),
                    egui::Layout::left_to_right(egui::Align::Center)
                );
                
                let stroke = egui::Stroke::new(1.0, egui::Color32::GOLD);

                let elements_button = tabs.add(egui::Button::new("Elements").stroke(stroke));
                let creation_button = tabs.add(egui::Button::new("Create").stroke(stroke));
                let load_button = tabs.add(egui::Button::new("Load").stroke(stroke));
                

                if elements_button.clicked()
                {
                    self.load_elements.set(true);
                    self.load_creation.set(false);
                }
                if creation_button.clicked()
                {
                    self.load_elements.set(false);
                    self.load_creation.set(true);
                }
                if load_button.clicked()
                {
                    self.selected = None;
                    self.load_database.set(true);
                }
            });

        if self.load_database.get()
        { 
            egui::Window::new("Load Database")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
                .resize(|r| {r.min_size(egui::Vec2::new(20.0, 10.0))})
                .show(ctx, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui|{
                        ui.add_space(10.0);                                                            
                        ui.set_width(ctx.available_rect().width());                                    
                        ui.set_height(ctx.available_rect().height());
                        egui::ComboBox::from_id_source("databases")
                            .selected_text(self.selected.as_deref().unwrap_or("None"))
                            .width(ctx.available_rect().width() / 2.0)
                            .show_ui(ui, |ui| {
                                for db in &self.databases {
                                    let selectable_value = ui.selectable_value(&mut self.selected, Some(db.clone()), db);
                                }
                          });  
                    });
                    
                });
        }

        if self.load_elements.get()
        {
            egui::Window::new("Elements")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
            .resize(|r| {r.min_size(egui::Vec2::new(20.0, 10.0))})
            .show(ctx, |ui| {
                let selected = self.selected.as_deref().clone();
                //TODO: Show the loaded elements
                self.load_elements.set(false);
            });
        }
        if self.selected.is_some()
        {
            self.load_database.set(false);
            let selected_ref = self.selected.as_deref().unwrap();
            env::set_var("DATABASE_PATH", format!("saves/{}",selected_ref));
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

