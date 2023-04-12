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
    conn: sqlite::Connection,
    databases: Vec<String>,
    selected: Option<String>
}

impl Default for TTRPGMaker
{
    fn default() -> Self
    {
        // Set the load database to true
        let load_database = std::cell::Cell::new(true);
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
        
        Self { load_database, conn, databases, selected }
    }
}

impl eframe::App for TTRPGMaker {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Create a UI with a label and a combo box
        frame.set_centered();
        egui::TopBottomPanel::top("tabs")
            .show_separator_line(false)
            .show(ctx, |ui| {
                let mut tabs = ui.child_ui(
                    ui.available_rect_before_wrap(),
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight)
                );
                
                let stroke = egui::Stroke::new(1.0, egui::Color32::GOLD);
                tabs.add(egui::Button::new("Element Creation").stroke(stroke))
            });

        if self.load_database.get()
        { 
            egui::Window::new("Load Database")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
                .resize(|r| {r.min_size(egui::Vec2::new(20.0, 10.0))})
                .show(ctx, |ui| {

                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui|{
                        ui.add_space(10.0);                                                            
                        ui.set_width(ctx.available_rect().width());                                    
                        ui.set_height(ctx.available_rect().height());
                        egui::ComboBox::from_id_source("databases")
                            .selected_text(self.selected.as_deref().unwrap_or("None"))
                            .width(ctx.available_rect().width() / 2.0)
                            .show_ui(ui, |ui| {
                                for db in &self.databases {
                                    let mut selectable_value = ui.selectable_value(&mut self.selected, Some(db.clone()), db);
                                }                                                                                                   
                          });  
                    });
                    
                });
        }
        if self.selected.is_some()
        {
            self.load_database.set(false);
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

