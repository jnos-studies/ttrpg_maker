use std::env;
use sqlite;
use entities::*;
use narratives::*

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
    pub fn new(name: &str) -> Returned_TTRPG
    {
        let connection = sqlite::Connection::open(env::var("DATABASE_PATH").unwrap()).unwrap();
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
            println!("{:?}", row[2].1.unwrap());
            if row[2].1.unwrap() == name {
                ttrpg.name = "Already Exists".to_string();
                ttrpg.id = row[0].1.unwrap().parse::<u32>().unwrap();
            }
            else
            {
                ttrpg.name = name.to_string();
                ttrpg.id = row[0].1.unwrap().parse::<u32>().unwrap();
            }
            true
        }).unwrap();

        if ttrpg.name != "Already Exists".to_string()
        {
            connection.execute(format!("INSERT INTO ttrpgs (name) VALUES ('{}')", name)).unwrap();
            ttrpg.name = String::from(name);
            return ttrpg
        }
        else
        {
            return ttrpg
        }
    }
    pub fn retrieve_existing(campaign_id: u32) -> Vec<Returned_TTRPG>
    {
        let connection = sqlite::Connection::open(env::var("DATABASE_PATH").unwrap()).unwrap();
        let ttrpgs = Vec::new();
        let stories = Vec::new();
        let attributes = Vec::new();
        let skills = Vec::new();
        let counters = Vec::new();
        let tables = Vec::new();

        connection.iterate("SELECT id FROM ttrpgs", |ttrpg| {
            let id = ttrpg[0].1.unwrap().parse::<u32>().unwrap();
            connection.iterate(format!("SELECT * FROM stories WHERE ttrpg_id = {}", id), |story| {
                let story_text = TypedNarrative::new(story[1].1.unwrap().to_string());
                let story_entity = entities::Story::new(story_text);
                stories.push(story_entity);
                true
            }).unwrap();

            true
        });
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
            ttrpg_id INTEGER NOT NULL,
            text_data TEXT NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpgs(id)
        );
    
        CREATE TABLE attributes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ttrpg_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpgs(id)
        );

        CREATE TABLE attribute_outcomes (
            attribute_id INTEGER NOT NULL,
            roll_description TEXT NOT NULL,
            base_result INTEGER NOT NULL,
            FOREIGN KEY (attribute_id) REFERENCES attributes(id)
        );
    
        CREATE TABLE rolls (
            ttrpg_id INTEGER NOT NULL,
            skill_id INTEGER PRIMARY KEY AUTOINCREMENT,
            blank_roll INTEGER NOT NULL,
            dice_label TEXT NOT NULL,
            dice INTEGER NOT NULL,
            amount INTEGER NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id)
        );

        CREATE TABLE skills (
            ttrpg_id INTEGER NOT NULL,
            roll_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id),
            FOREIGN KEY (roll_id) REFERENCES rolls(skill_id)
        );
  
        CREATE TABLE counters (
            ttrpg_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            number INTEGER NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id)
        );
  
        CREATE TABLE tables (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ttrpg_id INTEGER NOT NULL,
            description TEXT,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id)
        );
  
        CREATE TABLE table_values (
            table_id INTEGER NOT NULL,
            lower_range INTEGER NOT NULL,
            higher_range INTEGER NOT NULL,
            text_value TEXT NOT NULL,
            FOREIGN KEY (table_id) REFERENCES tables(id)
        );
    ";

    connection.execute(query).unwrap();
}

