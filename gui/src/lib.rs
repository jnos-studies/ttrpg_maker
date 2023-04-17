use eframe::egui;
use eframe::egui::TextBuffer;
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
                    self.load_database.set(false);
                }
                if creation_button.clicked()
                {
                    self.load_elements.set(false);
                    self.load_creation.set(true);
                    self.load_database.set(false);
                }
                if load_button.clicked()
                {
                    self.selected = None;
                    self.load_database.set(true);
                }
            });

        if self.load_database.get()
        { 
            egui::Window::new("Database and TTRPG creator")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.set_width(ctx.available_rect().width());
                    ui.set_height(ctx.available_rect().height() - 40.0);
                    // create child ui elements in the load page that will handle database
                    // creation and deletion, as well as the creation and deletion of
                    // ttrpg elements
                    let mut database_selection_and_creator = ui.child_ui
                    (
                        ui.max_rect(),
                        egui::Layout::top_down_justified(egui::Align::TOP)
                    );
                    let mut ttrpg_selection_and_creator = ui.child_ui
                    (
                        ui.max_rect().shrink2(egui::vec2(0.0, ui.available_height() / 11.0)),
                        egui::Layout::top_down_justified(egui::Align::Center)
                    );
                    database_selection_and_creator.set_width(ui.available_width());
                    database_selection_and_creator.set_height(ui.available_height() / 4.0);
                    ttrpg_selection_and_creator.set_width(ui.available_width());
                    ttrpg_selection_and_creator.set_height(ui.available_height());
                    
                    database_selection_and_creator.group(|ui|
                    {
                        ui.label("Select a database");
                        egui::ComboBox::from_id_source("databases")
                            .selected_text(self.selected.as_deref().unwrap_or("None"))
                            .show_ui(ui, |ui|
                            {
                                for db in &self.databases
                                {
                                    let selectable_value = ui.selectable_value(&mut self.selected, Some(db.clone()), db);
                                    if selectable_value.clicked()
                                    {
                                        ctx.request_repaint();
                                    }
                                }
                            });
                    });

                    ttrpg_selection_and_creator.group(|ui|
                    {
                        // load existing ttrpgs in database
                        if self.selected.is_some()
                        {
                            let database_path = format!("saves/{}", self.selected.as_deref().unwrap().as_str());
                            let load_names = store_rpg::get_existing_ttrpgs_from_database(&database_path);
                            for name in load_names
                            {
                                ui.push_id(&name, |ui|
                                {
                                    ui.label(&name);               
                                });
                            }
                        }
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

