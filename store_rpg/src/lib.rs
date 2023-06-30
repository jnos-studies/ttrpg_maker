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
    pub stories: Vec<(u32, Story)>,
    pub attributes: Vec<(u32, Attribute)>,
    pub skills: Vec<(u32, Skill)>,
    pub counters: Vec<(u32, Counter)>,
    pub tables: Vec<(u32, Table)>,
    pub rolls: Vec<(u32, Outcome)>
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
            tables: Vec::new(),
            rolls: Vec::new()
        };
        // Gets database names
        connection.iterate("SELECT * FROM ttrpgs", |row: &[(&str, Option<&str>)]| {
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
    // TODO Create loading for each type of ttrpg element
    pub fn load_elements(mut self, database_path: &str) -> Option<Returned_TTRPG>{
        let connection = sqlite::Connection::open(database_path).unwrap();
        connection.iterate(format!("SELECT * FROM stories WHERE ttrpg_id = {}", self.id), |row: &[(&str, Option<&str>)]| {
            self.stories.push(
                (
                    row[0].1.unwrap().parse().unwrap(),
                    Story::new(TypedNarrative::new(row[2].1.unwrap().to_string()))
                )
            );
            true
        }).unwrap();
        connection.iterate(format!("SELECT * FROM attributes WHERE ttrpg_id = {}", self.id),|row: &[(&str, Option<&str>)]| {
            self.attributes.push(
                (
                    row[0].1.unwrap().parse().unwrap(), 
                    Attribute::new(
                        TypedNarrative::new(row[2].1.unwrap().to_string()),
                        Outcome {
                            roll_description:  row[3].1.unwrap().to_string(),
                            base_result: row[4].1.unwrap().parse().unwrap(), 
                            max: 1, 
                            min: 1, 
                            attribute: true, 
                            critical: 20
                        }
                    )
                )
            );
            true
        }).unwrap();
        // TODO
        connection.iterate(format!("SELECT * FROM skills WHERE ttrpg_id = {}", self.id), |row: &[(&str, Option<&str>)]| { 
            self.skills.push(
                (
                    row[0].1.unwrap().parse().unwrap(),
                    Skill::new(
                        TypedNarrative::new(row[2].1.unwrap().to_string()),
                        Roll {
                            dice_label: row[3].1.unwrap().to_string(),
                            dice: row[4].1.unwrap().parse().unwrap(), 
                            amount: row[5].1.unwrap().parse().unwrap() 
                        }
                    )
                )
            );
            true
         }).unwrap();
        connection.iterate(format!("SELECT * FROM counters WHERE ttrpg_id = {}", self.id), |row: &[(&str, Option<&str>)]| { 
            self.counters.push(
                (
                    row[0].1.unwrap().parse().unwrap(),
                    Counter::new(
                        TypedNarrative::new(row[2].1.unwrap().to_string()),
                        row[3].1.unwrap().parse().unwrap()
                    )
                )
            );
            true 
        }).unwrap();

        connection.iterate(format!("SELECT * FROM tables WHERE ttrpg_id = {}", self.id), |row: &[(&str, Option<&str>)]| { 
            self.tables.push(
                (
                    row[0].1.unwrap().parse().unwrap(),
                    Table::new(
                        TypedNarrative::new(row[2].1.unwrap().to_string()),
                        TabledNarratives {
                            table: TabledNarratives::values_from_json(row[3].1.unwrap())
                        }
                    )
                )
            );

            true 
        }).unwrap();

        connection.iterate(format!("SELECT * FROM all_rolls WHERE ttrpg_id = {}", self.id), |row: &[(&str, Option<&str>)]| {
            self.rolls.push(
                (
                    row[0].1.unwrap().parse().unwrap(),
                    Outcome::new(
                        &Roll::new(
                            row[4].1.unwrap().parse().unwrap(),
                            row[5].1.unwrap().parse().unwrap()
                        ),
                        match env::var("CRITICAL_SETTING").unwrap().parse::<u32>().unwrap() {
                            20 => &Critical::Twenty,
                            1 => &Critical::One,
                            _ => &Critical::Twenty // Defaults to critical 20 dice type
                        },
                        row[6].1.unwrap().parse().unwrap(),
                        false
                    )
                )
            );
            
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
            bonus INTEGER NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id)
        );

        CREATE TABLE skills (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ttrpg_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            dice_label TEXT NOT NULL,
            dice INTEGER NOT NULL,
            amount INTEGER NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id)
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
            values_json TEXT NOT NULL,
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
        DELETE FROM attributes WHERE ttrpg_id = {};
        DELETE FROM all_rolls WHERE ttrpg_id = {};
        DELETE FROM skills WHERE ttrpg_id = {};
        DELETE FROM counters WHERE ttrpg_id = {};
        DELETE FROM tables WHERE ttrpg_id = {}; 
        ",
        ttrpg_id,
        ttrpg_id,
        ttrpg_id,
        ttrpg_id,
        ttrpg_id,
        ttrpg_id,
        ttrpg_id
        );

    match connection.execute(query) {
        Ok(_s) => println!("Deletion successful"),
        Err(e) => println!("{:#?}", e)
    }

        String::from(format!("TTRPG {} successfully deleted!", ttrpg_name))
}
