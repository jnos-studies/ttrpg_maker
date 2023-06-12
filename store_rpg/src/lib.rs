use std::env;
use sqlite;
use entities::*;
use narratives::*;
use roll_dice::*;

#[derive(Clone)]
pub struct Returned_TTRPG
{
    pub name: String,
    pub id: u32,
    pub stories: Vec<entities::Story>,
    pub attributes: Vec<entities::Attribute>,
    pub skills: Vec<entities::Skill>,
    pub counters: Vec<entities::Counter>,
    pub tables: Vec<entities::Table>
}

impl Returned_TTRPG
{
    pub fn new(name: &str, loading: bool) -> Option<Returned_TTRPG>
    {
        let connection = match sqlite::Connection::open(env::var("DATABASE_PATH").unwrap())
        {
            Ok(conn) => conn,
            Err(_) => {
                return None
            }
        };
        let mut exists = false;
        let mut ttrpg = Returned_TTRPG
        {
            name: "".to_string(),
            id: 0,
            stories: Vec::new(),
            attributes: Vec::new(),
            skills: Vec::new(),
            counters: Vec::new(),
            tables: Vec::new()
        };
        connection.iterate("SELECT * FROM ttrpgs", |row| {
            // simple debug println!("{:?}", row[2].1.unwrap());
            if row[2].1.unwrap() == name {
                ttrpg.name = name.to_string().clone();
                ttrpg.id = row[0].1.unwrap().parse::<u32>().unwrap().clone();
                exists = true;
            }
            true
        }).unwrap();

        if exists == false
        {
            connection.execute(format!("INSERT INTO ttrpgs (name) VALUES ('{}')", name)).unwrap();
            ttrpg.name = String::from(name);
            return Some(ttrpg.clone())
        }
        if loading == true
        {
            return Some(ttrpg)
        }
        None
    }
    pub fn load_elements(mut self, database_path: &str) -> Option<Returned_TTRPG>{
        let connection = sqlite::Connection::open(database_path).unwrap();
        connection.iterate(format!("SELECT * FROM stories WHERE ttrpg_id = {}", self.id), |row| {
            for (column, value) in row.iter() {
                if column.contains("text_data") {
                    let story = Story::new(TypedNarrative::new(value.unwrap().to_string().clone()));
                    println!("{:#?}", story);
                    self.stories.push(story);
                }
            }
            true
        }).unwrap();
    Some(self)
    }
}

pub fn database_setup(database_path: &str)
{ 
    let connection = sqlite::Connection::open(database_path).unwrap();
    let query = "CREATE TABLE ttrpgs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date DATETIME DEFAULT CURRENT_TIMESTAMP,
            name TEXT NOT NULL);

        CREATE TABLE stories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ttrpg_id INTEGER NOT NULL,
            text_data TEXT NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpgs(id)
        );
    
        CREATE TABLE attributes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ttrpg_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            roll_description TEXT NOT NULL,
            base_result TEXT NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpgs(id)
        );

        CREATE TABLE all_rolls (
            ttrpg_id INTEGER NOT NULL,
            blank_roll INTEGER NOT NULL,
            dice_label TEXT NOT NULL,
            dice INTEGER NOT NULL,
            amount INTEGER NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id)
        );

        CREATE TABLE skills (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ttrpg_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            dice_label TEXT NOT NULL,
            dice INTEGER NOT NULL,
            amount INTEGER NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id),
        );
  
        CREATE TABLE counters (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ttrpg_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            number INTEGER NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id)
        );
  
        CREATE TABLE tables (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ttrpg_id INTEGER NOT NULL,
            description TEXT,
            lower_range INTEGER NOT NULL,
            higher_range INTEGER NOT NULL,
            text_value TEXT NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id)
        ); 
    ";

    connection.execute(query).unwrap();
}

pub fn get_existing_ttrpgs_from_database(database_path: &str) -> Vec<String>
{
    let connection = sqlite::Connection::open(database_path).unwrap();
    let mut ttrpg_names = Vec::new();
    connection.iterate("SELECT name FROM ttrpgs", |row| {
        ttrpg_names.push(row[0].1.unwrap().to_string().clone());
        true
    }).unwrap();
    ttrpg_names
}
// Will panic
pub fn delete_ttrpg(database_path: &str, ttrpg_id: u32, ttrpg_name: &str) -> String
{
    let connection = sqlite::Connection::open(database_path).unwrap();
    let query = format!(
        "
        DELETE FROM ttrpgs WHERE id = {};
        DELETE FROM stories WHERE ttrpg_id = {};
        DELETE FROM attribute_outcomes WHERE attribute_id = (SELECT id FROM attributes WHERE id = {});
        DELETE FROM attributes WHERE ttrpg_id = {};
        DELETE FROM rolls WHERE ttrpg_id = {};
        DELETE FROM skills WHERE ttrpg_id = {};
        DELETE FROM counters WHERE ttrpg_id = {};
        DELETE FROM table_values WHERE table_id = (SELECT id FROM tables WHERE ttrpg_id = {});
        DELETE FROM tables WHERE ttrpg_id = {}; 
        ",
        ttrpg_id,
        ttrpg_id,
        ttrpg_id,
        ttrpg_id,
        ttrpg_id,
        ttrpg_id,
        ttrpg_id,
        ttrpg_id,
        ttrpg_id
        );

        connection.execute(query).unwrap();

        String::from(format!("TTRPG {} successfully deleted!", ttrpg_name))
}
