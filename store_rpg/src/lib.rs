use std::env;
use sqlite;
use entities::*;
use narratives::*;
use roll_dice::*;

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

    pub fn load_entity(&mut self)
    {
        let connection = sqlite::open(env::var("DATABASE_PATH").unwrap()).unwrap();
        let story_query = "
            SELECT text_data FROM stories WHERE ttrpg_id = ?";
        let attribute_query ="
            SELECT attributes.description, attribute_outcomes.roll_description, attribute_outcomes.base_result FROM attributes
            INNER JOIN attribute_outcomes
            ON attributes.id = attribute_outcomes.attribute_id
            WHERE attributes.id = ?";
        let skills_query = "
            SELECT skills.description, rolls.blank_roll, rolls.dice_label, rolls.dice, rolls.amount, FROM skills
            INNER JOIN rolls
            ON skills.roll_id = rolls.skill_id
            WHERE skills.ttrpg_id = ?";
        let counters_query = "
            SELECT description, number FROM counters WHERE ttrpg_id = ?";
        let tables_query = "
            SELECT tables.description, table_values.lower_range, table_values.higher_range, table_values.text_value FROM tables
            INNER JOIN table_values
            ON tables.id = table_values.table_id
            WHERE tables.ttrpg_id = ?";
        let mut prepare_stories = connection.prepare(story_query).unwrap();
        let mut prepare_attributes = connection.prepare(attribute_query).unwrap();
        let mut prepare_skills = connection.prepare(skills_query).unwrap();
        let mut prepare_counters = connection.prepare(counters_query).unwrap();
        let mut prepare_tables = connection.prepare(tables_query).unwrap();
        prepare_stories.bind((1, self.id as i64)).unwrap();
        prepare_attributes.bind((1, self.id as i64)).unwrap();
        prepare_skills.bind((1, self.id as i64)).unwrap();
        prepare_counters.bind((1, self.id as i64)).unwrap();
        prepare_tables.bind((1, self.id as i64)).unwrap();

        while let Ok(sqlite::State::Row) = prepare_stories.next()
        {
            let story_text = narratives::TypedNarrative::new(prepare_stories.read::<String,_>("text_data").unwrap().clone());
            self.stories.push(entities::Story::new(story_text));
        }
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

