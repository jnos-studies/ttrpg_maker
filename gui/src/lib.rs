use eframe::egui;
use eframe::egui::TextBuffer;
use std::env;
use store_rpg::*;
use roll_dice::{Critical, Outcome, Roll};
use std::collections::HashMap;
use sqlite;
use std::cell::Cell;
use uuid::Uuid;

pub struct TTRPGMaker {
    load_database: Cell<bool>,
    load_elements: Cell<bool>,
    view_edit: Cell<bool>,
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
        let view_edit = Cell::new(false);
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
            view_edit,
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
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                let stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
                let elements_button = tabs.add_sized(tabs_button_sizes, egui::Button::new("Elements").stroke(stroke));
                let load_button = tabs.add_sized(tabs_button_sizes, egui::Button::new("Load").stroke(stroke));
                let _tools_button = tabs.add_sized(tabs_button_sizes, egui::Button::new("Tools").stroke(stroke));
                
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
                                        self.elements.clear();
                                        self.loaded_ttrpg.clear();
                                        self.create_database = "".to_string();
                                        self.create_ttrpg = "".to_string();
                                        
                                        if self.selected.as_deref().unwrap() != "None".to_string()
                                        {
                                            let database_path = format!("saves/{}", self.selected.as_deref().unwrap().as_str());
                                            let load_names = store_rpg::get_existing_ttrpgs_from_database(&database_path);
                                            let default_check_box = Cell::new(false);
                                            for name in load_names
                                            {
                                                self.elements.insert(name.clone(), default_check_box.get().clone().into());
                                            }
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
                                        store_rpg::database_setup(&new_database.as_str()) //TODO make this function return a result to handle the error if database defaults to dummy
                                    }
                                }
                            }
                            if ui.button("Delete").clicked()
                            {
                                if self.selected.is_some() && self.selected.as_deref().unwrap() != "None".to_string()
                                {
                                    //Clear the the following when db is deleted also
                                    self.elements.clear();
                                    self.loaded_ttrpg.clear();
                                    self.create_database = "".to_string();
                                    self.create_ttrpg = "".to_string();

                                    let to_delete = format!("{}", self.selected.as_deref().unwrap());
                                    // Remove memory of database from the list of databases and the
                                    // physical file
                                    self.databases.retain(|db| *db != to_delete.to_string());
                                    self.databases.retain(|db| *db != "None".to_string()); // delete None bug if present
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
                            ui.horizontal(|ui| {
                                ui.text_edit_singleline(&mut self.create_ttrpg);
                                let create_button = egui::Button::new("Create");
                                if ui.add(create_button).clicked() &&
                                self.selected.is_some() &&
                                self.selected.as_deref().unwrap() != "None".to_string()
                                {
                                    // The created ttrpg element will be inserted into the database
                                    // if it doesn't already exists. So if created_ttrpg returns
                                    // None, that means that the ttrpg already exists
                                    let created_ttrpg = store_rpg::Returned_TTRPG::new(&self.create_ttrpg, false);
                                    if created_ttrpg.is_some() && !self.create_ttrpg.is_empty()
                                    {
                                        let ttrpg_value = created_ttrpg.unwrap();
                                        // push the name value onto the elements hash to load it
                                        self.elements.insert(ttrpg_value.name, std::cell::Cell::new(false));
                                        self.create_ttrpg = "".to_string();
                                    }
                                }
                            });
                        });
                        
                        // Handles the selection and deselection of ttrpg elements that get loaded
                        // later in the view_edit function.
                        if self.selected.is_some() && self.selected.as_deref().unwrap() != "None".to_string()
                        {
                            // Temp variable to hold the name of the element that needs to be
                            // removed from self.elements to delete the ui
                            let mut element_to_delete = "".to_string();
                            for (key, value) in self.elements.iter_mut()
                            {
                                // Add the delete and Checkbox selector
                                let check_val = &mut value.get();
                                let check_box = egui::Checkbox::new(check_val, key.clone());
                                let delete_button = egui::Button::new("Delete").small();
                                ui.horizontal(|ui|{
                                    if ui.add(check_box).clicked()
                                    {
                                        if value.get() == true
                                        {
                                            value.set(false);
                                            let _removed_val = self.loaded_ttrpg.remove(key.clone().as_str()); // Gets dropped
                                        }
                                        else
                                        {
                                            value.set(true);
                                            self.loaded_ttrpg.insert(
                                                key.clone(),
                                                store_rpg::Returned_TTRPG::new(key.as_str(), true).unwrap());
                                        }
                                    }
                                    // Delete from selected database
                                    if ui.add(delete_button).clicked()
                                    {
                                        let reload_ttrpg = store_rpg::Returned_TTRPG::new(key, true).unwrap();
                                        let current_db = env::var("DATABASE_PATH").unwrap();
                                        store_rpg::delete_ttrpg(
                                            &current_db,
                                            reload_ttrpg.id,
                                            &reload_ttrpg.name
                                        );
                                        self.loaded_ttrpg.remove(key);
                                        // figure out how to reload so that the ui elements delete when
                                        // deleted from the database
                                        element_to_delete = reload_ttrpg.name;
                                    }
                                });
                            }
                            //Remove the element from self.elements which then removes the checkbox
                            //and delete button from the ui
                            if element_to_delete.len() > 0
                            {
                                self.elements.remove(&element_to_delete);
                            }
                        }
                    });
                });
        }

        if self.load_elements.get()
        {
            egui::SidePanel::left("Elements")
                .show(ctx, |ui| {
                ui.set_width(ctx.available_rect().width() / 5.0);
                let scroll_area = egui::ScrollArea::vertical();
                scroll_area.show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    for (key, value) in self.loaded_ttrpg.iter_mut()
                    {
                        value.load_entity();
                        ui.collapsing(key, |ui|
                        {
                            // Selected ttrpgs get loaded here, but only to count the number of
                            // elements in it. This is not that efficient as it gets loaded twice
                            // essentially, but atm it is just to make it work.
                            ui.horizontal_top(|ui|
                            {
                                let mut reload_to_view = store_rpg::Returned_TTRPG::new(value.name.clone().as_str(), true).unwrap();
                                // reload the values
                                reload_to_view.load_entity();
                            
                                // True means to view and false means to edit
                                if ui.add_sized(egui::vec2(ui.available_width() / 3.0, 30.0), egui::Button::new("View")).clicked()
                                {
                                    self.view_edit.set(true);
                                }
                            
                                if ui.add_sized(egui::vec2(ui.available_width() / 3.0, 30.0), egui::Button::new("Edit")).clicked()
                                {
                                    self.view_edit.set(false);
                                }
                            });
                            let ttrpg_info = format!("Element Overview\n\nStories: {}\nAttributes: {}\nSkills: {}\nCounters:{}\nTables: {}",
                                value.stories.len(),
                                value.attributes.len(),
                                value.skills.len(),
                                value.counters.len(),
                                value.tables.len()
                            );
                            ui.strong(ttrpg_info);
                        });
                    }
                });
            });
        }

        if self.view_edit.get()
        {
            // for loop for debugging purposes
            let clone_loaded_ttrpgs = self.loaded_ttrpg.clone();
            for (key, val) in self.loaded_ttrpg.iter() {
                println!("ttrpg name: {}, val: {:#?}", key, val.id);
            }
            view_or_edit(&self.view_edit.get(), clone_loaded_ttrpgs, ctx);
        }


        if self.selected.is_some()
        {
            let selected_ref = self.selected.as_deref().unwrap();
            env::set_var("DATABASE_PATH", format!("saves/{}",selected_ref));
        }
    }
}

// If the variable bool view_edit = true the ui will generate a view of the data, Edit will
// generate an editing view of the data
fn view_or_edit(view_edit: &bool, hashmap_rpgs: HashMap<String, Returned_TTRPG>, ctx: &egui::Context)
{
    //TODO create a unique id for the SidePanel or use a different egui struct to push UIs with
    //unique ids
    let iterative_hash = hashmap_rpgs.iter();
    for (key, val) in iterative_hash { 
    egui::SidePanel::right("view_or_edit")
        .show(ctx, |ui| {
            ui.set_width(ui.available_width());
            ui.heading(key);
            ui.vertical(|ui| {
                ui.label("Stories");
                ui.horizontal_top(|ui| {
                    for story in &val.stories
                    {
                        if *view_edit {
                            ui.collapsing(story.summarized.summary.get(&0).unwrap().text.clone(), |ui| {
                            // Get a summary as a header
                            ui.strong(&story.raw_narration);
                            });
                        }
                        else
                        {
                            
                        }
                    }
                });
                ui.label("Attributes");
                ui.horizontal_top(|ui| {
                    for attribute in &val.attributes
                    {
                        if *view_edit
                        {
                            ui.collapsing(attribute.description.text.clone(), |ui| {
                                let outcome = attribute.attribute.clone();
                                ui.strong(outcome.roll_description);
                                ui.label("Base Result: ");
                                let base_result = format!("Base Result: {}", outcome.base_result.clone());
                                ui.strong(base_result);
                            });
                        }
                        else
                        {
                                
                        }
                    }
                });
                ui.label("Skills");
                ui.horizontal_top(|ui| {
                    for skill in &val.skills
                    {
                        if *view_edit
                        {
                            let skill_copy = skill.clone();
                            ui.collapsing(skill_copy.description.text, |ui| {
                                ui.label(skill_copy.roll.dice_label);
                                if ui.small_button("Roll skill").clicked()
                                {
                                    //let outcome_of_roll = Outcome::new(&skill_copy.roll,)
                                }
                            });
                        }
                        else
                        {
                            
                        }
                    }
                });
                ui.label("Counters");
                ui.horizontal_top(|_ui| {
                        
                });
                ui.label("Tables");
                ui.horizontal_top(|_ui| {
                        
                });
            });
        });
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

