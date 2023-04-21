use eframe::egui;
use eframe::egui::TextBuffer;
use std::env;
use store_rpg::*;
use sqlite;
use std::collections::HashMap;
use std::cell::Cell;



pub struct TTRPGMaker
{
    load_database: Cell<bool>,
    load_elements: Cell<bool>,
    create_database: String,
    create_ttrpg: String,
    conn: sqlite::Connection,
    databases: Vec<String>,
    selected: Option<String>,
    elements: HashMap<String, Cell<bool>>,
    loaded_ttrpg: HashMap<String, Returned_TTRPG>
}


impl Default for TTRPGMaker
{
    fn default() -> Self
    {
        // Set the load database to true and the other window bools to false
        let load_database = Cell::new(true);
        let load_elements = Cell::new(false);
        let create_database = "".to_string();
        let create_ttrpg = "".to_string();
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

        // elements hashmap for determining what should be loaded onto self.loaded_ttrpg
        let elements = HashMap::new();
        let loaded_ttrpg = HashMap::new();
        
        Self
        {
            load_database,
            load_elements,
            create_database,
            create_ttrpg,
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
                let mut tabs = ui.child_ui(
                    ui.available_rect_before_wrap(),
                    egui::Layout::left_to_right(egui::Align::Center)
                );
                tabs.spacing_mut().item_spacing = egui::Vec2::new(0.0, 0.0);
                let tabs_button_sizes = egui::Vec2::new(tabs.available_width() / 3.0, tabs.available_height());
                let stroke = egui::Stroke::new(1.0, egui::Color32::GOLD);
                let elements_button = tabs.add_sized(tabs_button_sizes, egui::Button::new("Elements").stroke(stroke));
                let load_button = tabs.add_sized(tabs_button_sizes, egui::Button::new("Load").stroke(stroke));
                
                if elements_button.clicked()
                {
                    self.load_elements.set(true);
                    self.load_database.set(false);
                }
                if load_button.clicked()
                {
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
                        ui.max_rect().shrink2(egui::vec2(0.0, ui.available_height() / 8.2)),
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
                                        // Clear the elements hash booleans
                                        self.elements.clear();
                                        let database_path = format!("saves/{}", self.selected.as_deref().unwrap().as_str());
                                        let load_names = store_rpg::get_existing_ttrpgs_from_database(&database_path);
                                        let default_check_box = Cell::new(false);
                                        for name in load_names
                                        {
                                            self.elements.insert(name.clone(), default_check_box.get().clone().into());
                                        }
                                    }
                                }
                            });
                        ui.horizontal(|ui|
                        {
                            let new_db_path = format!("{}.db", self.create_database);
                            if ui.button("Create new").clicked()
                            {
                                if !self.create_database.is_empty() &&
                                !self.create_database.contains(char::is_whitespace)
                                {
                                    let new_database = format!("saves/{}.db", self.create_database);
                                    let current_dbs = std::fs::read_dir("saves/").unwrap();
                                    let mut paths = Vec::new();

                                    for p in current_dbs
                                    {
                                        paths.push(p.unwrap().path().display().to_string());
                                    }

                                    if !paths.contains(&new_database.to_string())
                                    {
                                        self.databases.push(new_db_path.clone());
                                        store_rpg::database_setup(&new_database.as_str())
                                    }
                                }
                            }
                            if ui.button("Delete").clicked()
                            {
                                if self.selected.is_some() && self.selected.as_deref().unwrap() != "None"
                                {
                                    let to_delete = format!("{}", self.selected.as_deref().unwrap());
                                    // Remove memory of database from the list of databases and the
                                    // physical file
                                    self.databases.retain(|db| *db != to_delete.to_string());
                                    std::fs::remove_file(format!("saves/{}", to_delete)).unwrap();
                                    self.selected = Some("None".to_string());
                                }
                            }
                            ui.text_edit_singleline(&mut self.create_database);
                        });

                    });

                    ttrpg_selection_and_creator.group(|ui|
                    {
                        ui.horizontal(|ui| {
                            ui.heading("TTRPG Creator");

                            ui.label("Name: ");
                            ui.horizontal_top(|ui| {
                                ui.text_edit_singleline(&mut self.create_ttrpg);
                                let create_button = egui::Button::new("Create");
                                if ui.add(create_button).clicked()
                                {
                                    // The created ttrpg element will be inserted into the database
                                    // if it doesn't already exists. So if created_ttrpg returns
                                    // None, that means that the ttrpg already exists
                                    let created_ttrpg = store_rpg::Returned_TTRPG::new(&self.create_ttrpg, false);
                                    if created_ttrpg.is_some()
                                    {
                                        let ttrpg_value = created_ttrpg.unwrap();
                                        // push the name value onto the elements hash to load it
                                        self.elements.insert(ttrpg_value.name, std::cell::Cell::new(false));
                                    }
                                }
                            });
                        });

                        if self.selected.is_some()
                        {
                            for (key, value) in self.elements.iter_mut()
                            {
                                if ui.add(egui::Checkbox::new(&mut value.get(), key)).clicked()
                                {
                                    if value.get() == true
                                    {
                                        value.set(false);
                                        let _removed_val = self.loaded_ttrpg.remove(key); // Gets dropped
                                    }
                                    else
                                    {
                                        value.set(true);
                                        self.loaded_ttrpg.insert(
                                            key.clone(),
                                            store_rpg::Returned_TTRPG::new(key.as_str(), true).unwrap());
                                    }
                                }
                            }
                        }
                    });
                });
        }

        if self.load_elements.get()
        {
            egui::SidePanel::left("Elements")
                .show(ctx, |ui| {
                ui.set_width(ctx.available_rect().width()/ 4.0);
                for (key, value) in self.loaded_ttrpg.iter_mut()
                {
                    value.load_entity();
                    ui.collapsing(key, |ui|
                    {
                        ui.horizontal_top(|ui|
                        {
                            let ttrpg_info = format!("Element Overview\n\nStories: {}\nAttributes: {}\nSkills: {}\nCounters:{}\nTables: {}",
                                value.stories.len(),
                                value.attributes.len(),
                                value.skills.len(),
                                value.counters.len(),
                                value.tables.len()
                            );
                            // TODO: Create a hashmap of viewing Cells to track and close the
                            // closing buttons of the generated viewing windows. could do the
                            // same for creation windows.
                            let mut reload_to_view = store_rpg::Returned_TTRPG::new(value.name.clone().as_str(), true).unwrap();
                            // reload the values
                            reload_to_view.load_entity();
                             if ui.add_sized(egui::vec2(ui.available_width() / 2.0, 30.0), egui::Button::new("View")).clicked()
                            {
                                // TODO: Need to add a viewing page
                            }
                            
                            ui.small(ttrpg_info);
                        });
                    });
                }
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

