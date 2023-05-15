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
                ttrpg.name = name.to_string();
                ttrpg.id = row[0].1.unwrap().parse::<u32>().unwrap();
                exists = true;
            }
            true
        }).unwrap();

        if exists == false
        {
            connection.execute(format!("INSERT INTO ttrpgs (name) VALUES ('{}')", name)).unwrap();
            ttrpg.name = String::from(name);
            return Some(ttrpg)
        }
        if loading == true
        {
            ttrpg.load_entity();
            return Some(ttrpg)
        }
        None
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
            SELECT skills.description, rolls.dice, rolls.amount FROM skills
            INNER JOIN rolls
            ON skills.roll_id = rolls.skill_id
            WHERE skills.ttrpg_id = ?";
        let counters_query = "
            SELECT description, number FROM counters WHERE ttrpg_id = ?";
        let tables_query = "
            SELECT id, description FROM tables WHERE ttrpg_id = ? ";
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

        let mut tables_to_search = Vec::new();
        
        while let Ok(sqlite::State::Row) = prepare_stories.next()
        {
            let story_text = narratives::TypedNarrative::new(prepare_stories.read::<String,_>("text_data").unwrap().clone());
            self.stories.push(entities::Story::new(story_text));
        }
        while let Ok(sqlite::State::Row) = prepare_attributes.next()
        {
            let (attribute_text, roll_description, base_result) =
            (
                prepare_attributes.read::<String, _>("attributes.description").unwrap().clone(),
                prepare_attributes.read::<String, _>("attribute_outcomes.roll_description").unwrap().clone(),
                prepare_attributes.read::<i64, _>("attribute_outcomes.base_result").unwrap().clone() 
            );
            let text_to_typed_narrative = narratives::TypedNarrative::new(attribute_text);
            let outcome = roll_dice::Outcome {
                roll_description,
                base_result: base_result as u32,
                max: 0,
                min: 0,
                attribute: true,
                critical: 1 //roll_dice::Critical::One // TODO: change this to be an environment variable the user can select.
            };
            self.attributes.push(entities::Attribute::new(text_to_typed_narrative, outcome));
        }
        while let Ok(sqlite::State::Row) = prepare_skills.next()
        {
            let (description, dice, amount) =
            (
                prepare_skills.read::<String, _>("skills.description").unwrap(),
                prepare_skills.read::<i64, _>("rolls.dice").unwrap() as u32,
                prepare_skills.read::<i64, _>("rolls.amount").unwrap() as u32,
            );

            let text_to_typed_narrative = narratives::TypedNarrative::new(description);
            let roll = roll_dice::Roll::new(dice, amount);
            self.skills.push(entities::Skill::new(text_to_typed_narrative, roll));
        }
        while let Ok(sqlite::State::Row) = prepare_counters.next()
        {
            let (description, number) =
            (
                prepare_counters.read::<String, _>("description").unwrap(),
                prepare_counters.read::<i64, _>("number").unwrap() as u32
            );
            let text_to_typed_narrative = narratives::TypedNarrative::new(description);
            self.counters.push(entities::Counter::new(text_to_typed_narrative, number));
        }
        while let Ok(sqlite::State::Row) = prepare_tables.next()
        {
            let (id, description) =
            (
               prepare_tables.read::<i64, _>("id").unwrap() as u32,
               prepare_tables.read::<String, _>("description").unwrap(),
            );
            tables_to_search.push((id, description));
        }

        // For every table, load there values from the database. This creates a new query for every
        // table and is not very efficient for database access. But as long as it works for now
        // that is okay
        for table in tables_to_search
        {
            let table_values_query =
                "SELECT lower_range, higher_range, text_value FROM table_values
                WHERE table_id = ?";
            let mut prepare_table_values = connection.prepare(table_values_query).unwrap();
            prepare_table_values.bind((1, table.0 as i64)).unwrap();
            
            let mut values: Vec<((u32, u32), String)> = Vec::new();
            while let Ok(sqlite::State::Row) = prepare_table_values.next()
            {
                let (lower_range, higher_range, text_value) =
                (
                    prepare_table_values.read::<i64, _>("lower_range").unwrap() as u32,
                    prepare_table_values.read::<i64, _>("higher_range").unwrap() as u32,
                    prepare_table_values.read::<String, _>("text_value").unwrap(),
                );
                values.push(((lower_range, higher_range), text_value));
            }

            let text_to_typed_narrative = narratives::TypedNarrative::new(table.1);
            let table_to_tabled_narratives = narratives::TabledNarratives::new(values);
            self.tables.push(entities::Table::new(text_to_typed_narrative, table_to_tabled_narratives));
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
            id INTEGER PRIMARY KEY AUTOINCREMENT,
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
