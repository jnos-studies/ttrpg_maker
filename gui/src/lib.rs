use eframe::egui;
use eframe::egui::TextBuffer;
use entities::{Story, Attribute, Skill, Counter, Table, ElementsEnum, SaveLoad};
use narratives::TypedNarrative;
use std::env;
use store_rpg::*;
use roll_dice::{Critical, Outcome, Roll};
use std::collections::HashMap;
use sqlite;
use std::cell::Cell;


pub struct TTRPGMaker {
    load_database: Cell<bool>,
    load_elements: Cell<bool>,
    view_edit: Cell<bool>,
    selected_el: (String, u32),
    create_database: String,
    create_ttrpg: String,
    conn: sqlite::Connection,
    databases: Vec<String>,
    selected: Option<String>,
    elements: HashMap<String, Cell<bool>>,
    loaded_ttrpg: HashMap<String, Returned_TTRPG>,
    new_text: String,
    edit_el: (u32, <Story as SaveLoad>::Entity),
    create_el: String,
    selected_ttrpg: Vec<(u32, ElementsEnum)>,
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
        let selected_el = ("".to_string(), 0);
        let elements = HashMap::new();
        let loaded_ttrpg = HashMap::new();
        let new_text = "".to_string();
        let edit_el = (0 as u32, Story::new(TypedNarrative::new("".to_string())));
        let create_el = "Story".to_string();
        let selected_ttrpg = Vec::new();
        
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
            selected_el,
            elements,
            loaded_ttrpg,
            new_text,
            edit_el,
            create_el,
            selected_ttrpg,
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
                                            let _removed_val = self.loaded_ttrpg.remove(key.clone().as_str());
                                            //TODO add the values from this removed val to a log
                                            //system that has yet to be created.
                                        }
                                        else
                                        {
                                            value.set(true);
                                            let load_the_ttrpg_el = reload_ttrpg(key.as_str(), &self.selected);
                                            self.loaded_ttrpg.insert(key.clone(), load_the_ttrpg_el);
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
                        ui.collapsing(key, |ui|
                        {
                            // Selected ttrpgs get loaded here, but only to count the number of
                            // elements in it. This is not that efficient as it gets loaded twice
                            // essentially, but atm it is just to make it work.
                            ui.horizontal_top(|ui|
                            {
                                let button = egui::Button::new("View");
                                let size_button = egui::vec2(ui.available_width() / 3.0, 30.0);
                                if ui.add_sized(size_button, button).clicked() {
                                    // Map each selected element to its values loaded from the
                                    // database el.0 is the id and el.1 is the value itself.
                                    self.view_edit.set(true);
                                    self.selected_el = (value.name.clone(), value.id.clone());
                                    self.selected_ttrpg = value.stories.clone()
                                        .into_iter()
                                        .map(|el|(el.0, ElementsEnum::Story(el.1)))
                                        .collect();
                                    self.selected_ttrpg.extend(
                                        value.attributes
                                        .clone()
                                        .into_iter()
                                        .map(|el| (el.0, ElementsEnum::Attribute(el.1)))
                                    );
                                    self.selected_ttrpg.extend(
                                        value.skills
                                        .clone()
                                        .into_iter()
                                        .map(|el| (el.0, ElementsEnum::Skill(el.1)))
                                    );
                                    self.selected_ttrpg.extend(
                                        value.counters
                                        .clone()
                                        .into_iter()
                                        .map(|el| (el.0, ElementsEnum::Counter(el.1)))
                                    );
                                    self.selected_ttrpg.extend(
                                        value.tables
                                        .clone()
                                        .into_iter()
                                        .map(|el| (el.0, ElementsEnum::Table(el.1)))
                                    );
                                }
                                if ui.add_sized(size_button, egui::Button::new("Edit")).clicked() {
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
        //Is constantly checking if it needs to save a story

        if self.view_edit.get() // If true or false
        {
            egui::TopBottomPanel::top("Creation Panel")
                .show(ctx, |ui| {
                    let creation_panel_display = if Some(self.selected_el.1).is_some() {
                        format!("Creation Panel (Selected: {})", &self.selected_el.0)
                    }
                    else {
                        format!("Creation Panel (Selected: {})", "None")
                    };
                    ui.collapsing(creation_panel_display, |ui| {
                        ui.horizontal_top(|ui| {
                            if ui.button("Story").clicked() {
                                self.create_el = String::from("Story");
                            }
                            if ui.button("Attribute").clicked() {
                                self.create_el = String::from("Attribute");
                            }
                            if ui.button("Skill").clicked() {
                                self.create_el = String::from("Skill");
                            }
                            if ui.button("Counter").clicked() {
                                self.create_el = String::from("Counter");
                            }
                            if ui.button("Table").clicked() {
                                self.create_el = String::from("Table");
                            }
                        });
                        //TODO generate different element creation contexts based on the button clicks
                        ui.text_edit_multiline(&mut self.new_text);
                        if ui.button("Save").clicked() && Some(&self.selected_el.0).is_some() {
                            let db = format!("saves/{}", self.selected.as_deref().unwrap());
                            let new_story = Story::new(TypedNarrative::new(self.new_text.clone()));
                            self.new_text.clear();
                            new_story.save(db.as_str(), self.selected_el.1.clone()).expect("Did not save damnit!");
                            println!("ID of saved story {}", self.selected_el.1);
                            for key in self.loaded_ttrpg.clone().into_iter() {
                                if key.0 == self.selected_el.0 {
                                    let key_string = key.0.as_str();
                                    let reloaded_ttrpg = reload_ttrpg(key_string, &self.selected);
                                    self.loaded_ttrpg.remove(key_string);
                                    self.loaded_ttrpg.insert(key.0, reloaded_ttrpg);
                                    self.selected_ttrpg.clear();
                                }
                            }
                        }
                    });
                });
            egui::SidePanel::right("View_Edit")
                .exact_width(ctx.available_rect().width())
                .show(ctx, |ui| {
                    ui.label("Your existing stories");
                     let _result = generate_view_edit(
                         ui, 
                         &mut self.view_edit.get(),
                         &mut self.selected_ttrpg, 
                         &mut self.edit_el,
                         format!("saves/{}", self.selected.as_deref().unwrap()).as_str(),
                         ).unwrap();
                });
        }
        else if self.view_edit.get() == false && self.selected.is_some() {
            let creation_panel_display = if Some(self.selected_el.1).is_some() {
                format!("Creation Panel (Selected: {})", &self.selected_el.0)
            }
            else {
                format!("Creation Panel (Selected: {})", "None")
            };
            egui::TopBottomPanel::top("Creation Panel")
                .show(ctx, |ui| {
                    ui.collapsing(creation_panel_display, |ui| {
                        ui.text_edit_multiline(&mut self.new_text);
                        if ui.button("Save").clicked() {
                            let db = format!("saves/{}", self.selected.as_deref().unwrap());
                            let new_story = Story::new(TypedNarrative::new(self.new_text.clone()));
                            self.new_text.clear();
                            new_story.save(db.as_str(), self.selected_el.1.clone()).expect("Did not save damnit!");
                            for key in self.loaded_ttrpg.clone().into_iter() {
                                if key.0 == self.selected_el.0 {
                                    let key_string = key.0.as_str();
                                    let reloaded_ttrpg = reload_ttrpg(key_string, &self.selected);
                                    self.loaded_ttrpg.remove(key_string);
                                    self.loaded_ttrpg.insert(key.0, reloaded_ttrpg);
                                    self.selected_ttrpg.clear();
                                }
                            }
                        }
                    });
                });
            egui::SidePanel::right("View_Edit")
                .exact_width(ctx.available_rect().width())
                .show(ctx, |ui| {
                    ui.label("Your existing stories");
                     let result = generate_view_edit(
                         ui, 
                         &mut self.view_edit.get(),
                         &mut self.selected_ttrpg, 
                         &mut self.edit_el,
                         format!("saves/{}", self.selected.as_deref().unwrap()).as_str(),
                         ).unwrap();
                     // if result is ever true, reload the selected ttrpg (repetative code)
                     if result {
                        let clone_key = self.selected_el.0.clone();
                        self.loaded_ttrpg.remove(&clone_key);
                        let load_the_ttrpg_el = reload_ttrpg(clone_key.as_str(), &self.selected);
                        self.loaded_ttrpg.insert(clone_key, load_the_ttrpg_el);
                        self.selected_ttrpg.clear();
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

// Helper function to reload Returned_TTRPG into ui from database
fn reload_ttrpg (key: &str, db_selected_path: &Option<String>) -> Returned_TTRPG {
    let load_the_ttrpg_el = store_rpg::Returned_TTRPG::new(key, true).unwrap().clone();
    let db = format!("saves/{}",db_selected_path.as_deref().unwrap());
    let load_the_ttrpg_el = load_the_ttrpg_el.load_elements(db.as_str()).unwrap();
    load_the_ttrpg_el
}
// Helper function for view_edit that returns the ui to generate depending on conditions provided
// by the bool check
fn generate_view_edit(ui: &mut egui::Ui, view: &mut bool, selected_enum_vector: &mut Vec<(u32, ElementsEnum)>, edit_el: &mut (u32, <Story as SaveLoad>::Entity), db_path: &str) -> Option<bool> {
    let mut result: bool = false;
    for elem in selected_enum_vector {
        match (elem.0, elem.1.clone()) {
            (id ,ElementsEnum::Story(mut s)) => {
                //TODO Future development is to use a Summarizer to summarize a large body of text
                //into a sentence.
                let story_summary = &s.raw_narration[0..(s.raw_narration.len() / 3)].to_string();
                ui.push_id(format!("{} - {}", &id, story_summary.clone()), |ui| {
                    let view_text = egui::RichText::new(story_summary).size(14.0);
                    let view_text_raw = egui::RichText::new(&s.raw_narration.clone()).size(14.0);
                    if *view == true {
                        let collapsing_ui = egui::CollapsingHeader::new(view_text);
                        if collapsing_ui.show(ui, |ui| {
                            ui.strong(view_text_raw);
                        }).fully_open() {
                            *edit_el = (id, s.clone());
                        }
                    }
                    else if &edit_el.0 == &id {
                        ui.strong(view_text);
                        ui.text_edit_multiline(&mut edit_el.1.raw_narration);
                        //println!("Gained focus: ID: {}", &id);
                        ui.horizontal(|ui| {
                            if ui.button("Save and Update").clicked() {
                                let update_entity = Story::new(TypedNarrative::new(edit_el.1.raw_narration.clone()));
                                s.update(db_path, id.clone(), update_entity.clone()).unwrap();
                                s = update_entity;
                                result = true;
                                edit_el.0 = 0;
                            }
                            if ui.button("Delete Story Element").clicked() {
                                s.delete(db_path, id).expect("Did not delete story element");
                                result = true;
                                edit_el.0 = 0;
                            }
                        });
                    }
                });
            },
            (id, ElementsEnum::Attribute(mut a)) => {
                let description = &a.description.text;
                ui.push_id(format!("{} - {}", &id, description), |ui| {

                });
            },
            (id, ElementsEnum::Skill(s)) => {},
            (id, ElementsEnum::Counter(c)) => {},
            (id, ElementsEnum::Table(t)) => {}
        }
    }
    Some(result)
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

