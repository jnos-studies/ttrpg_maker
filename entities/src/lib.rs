use roll_dice::*;
use narratives::*;
use sqlite;
use eframe::egui::TextBuffer;
// Story
#[derive(Clone, Debug)]
pub struct Story {
    pub raw_narration: String,
    pub summarized: AutoNarrative
}
//TODO define TextBuffer for every entity
impl Story {
     pub fn new(raw_narration: TypedNarrative) -> Story {
         Story {
            raw_narration: raw_narration.text.to_owned(),
            summarized: AutoNarrative::new(raw_narration)
        }
     }
}

impl TextBuffer for Story {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        &self.raw_narration
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        self.raw_narration.insert_str(char_index, text);
        char_index + text.len()
    }
    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        let start = char_range.start;
        let end = char_range.end;
        self.raw_narration.replace_range(start..end, "");
    }
}
// Attribute
#[derive(Clone, Debug)]
pub struct Attribute {
    pub description: TypedNarrative,
    pub attribute: Outcome,
}

impl Attribute {
    pub fn new (description: TypedNarrative, attribute: Outcome) -> Attribute {
        if attribute.attribute {
            Attribute {
                description,
                attribute
            }
        } else {
            panic!("Unable to create attribute: attribute set to false")
        }
    }
}


// Skill
#[derive(Clone, Debug)]
pub struct Skill {
    pub description: TypedNarrative,
    pub roll: Roll
}

impl Skill {
    pub fn new(description: TypedNarrative, roll: Roll) -> Skill {
        Skill {
            description,
            roll
        }
    }
}

// Counter
#[derive(Clone, Debug)]
pub struct Counter {
    pub description: TypedNarrative,
    pub number: u32
}

impl Counter {
    pub fn new(description: TypedNarrative, number: u32) -> Counter {
        Counter {
            description,
            number
        }
    }

    pub fn change_number(&mut self, new_number: u32) {
        self.number = new_number;
    }
}
// Table
#[derive(Clone, Debug)]
pub struct Table {
    pub description: TypedNarrative,
    pub table: TabledNarratives
}

impl Table {
    pub fn new(description: TypedNarrative, table: TabledNarratives) -> Table {
        Table {
            description,
            table
        }
    }
}

/// Implement save method for all entity object. Entity id for the update function are accessed when
/// loading the values from the database into the UI
pub trait SaveLoad {
    type Entity;
    fn save(&self, database_path: &str, campaign_id: u32) -> Result<(), String>;
    fn update(&self, database_path: &str, entity_id: u32, update_entity: Self::Entity) -> Result<(), String>;
    fn delete(&self, database_path: &str, entity_id: u32) -> Result<(), String>;
}

impl SaveLoad for Story {
    type Entity = Story;
    fn save(&self, database_path: &str, campaign_id: u32) -> Result<(), String>
    {
        let connection = sqlite::Connection::open(database_path).unwrap(); 
        let query = format!(
            "
            INSERT INTO stories (
                ttrpg_id,
                text_data
            )
            VALUES ({}, '{}')
            ",
            campaign_id,
            self.raw_narration
        );
        
        connection.execute(query).unwrap();
        Ok(())
    }
    fn update(&self, database_path: &str, entity_id: u32, update_entity: Story) -> Result<(), String>
    {
        let connection = sqlite::Connection::open(database_path).unwrap();
        let query = format!(
            "UPDATE stories SET text_data = '{}' WHERE story_id = {};",
            update_entity.raw_narration,
            entity_id
        );
        connection.execute(query).unwrap();
        Ok(())
    }
    fn delete(&self, database_path: &str, entity_id: u32) -> Result<(), String>
    {
        let connection = sqlite::Connection::open(database_path).unwrap();
        let query = format!("DELETE FROM stories WHERE story_id = {};", entity_id);
        connection.execute(query).unwrap();
        Ok(())
    }
}

impl SaveLoad for Attribute {
    type Entity = Attribute;
    fn save(&self, database_path: &str, campaign_id: u32) -> Result<(), String>
    {
        let connection = sqlite::Connection::open(database_path).unwrap(); 
        let query_attribute = format!(
            "
            INSERT INTO attributes (
                ttrpg_id,
                description,
                roll_description,
                base_result
            )
            VALUES ({}, '{}', '{}', '{}')
            ",
            campaign_id,
            self.description.text,
            self.attribute.roll_description,
            self.attribute.base_result
        );
        connection.execute(query_attribute).unwrap();
        Ok(())
    }

    fn update(&self, database_path: &str, entity_id: u32, update_entity: Self::Entity) -> Result<(), String>
    {
        let connection = sqlite::Connection::open(database_path).unwrap();
        // Remove and add a new attribute to replace the old one
        let query = format!(
            "
            UPDATE attributes SET description = '{}', roll_description = '{}, base_result = {} WHERE id = {};
            ",
            update_entity.description.text,
            update_entity.attribute.roll_description,
            update_entity.attribute.base_result,
            entity_id
        );

        connection.execute(query).unwrap();
        Ok(())
        
    }
    fn delete(&self, database_path: &str, entity_id: u32) -> Result<(), String>
    {
        let connection = sqlite::Connection::open(database_path).unwrap();
        let query = format!(
            "
            DELETE FROM attributes WHERE id = {};
            ",
            entity_id
        );
        connection.execute(query).unwrap();
        Ok(())
    }
}

impl SaveLoad for Skill {
    type Entity = Skill;
    fn save(&self, database_path: &str, campaign_id: u32) -> Result<(), String>
    {
        let connection = sqlite::Connection::open(database_path).unwrap(); 
        let query_roll = format!(
            "
                INSERT INTO skills (
                    ttrpg_id,
                    description,
                    dice_label,
                    dice,
                    amount
                ) VALUES ({}, '{}', '{}', {}, {})
            ",
            campaign_id,
            self.description.text,
            self.roll.dice_label,
            self.roll.dice,
            self.roll.amount
        );
        connection.execute(query_roll).unwrap();
        Ok(())
    }
    fn update(&self, database_path: &str, entity_id: u32, update_entity: Self::Entity) -> Result<(), String>
    {
        let connection = sqlite::Connection::open(database_path).unwrap();
        let query = format!(
            "
            UPDATE skills SET description = '{}', dice_label = '{}', dice = {}, amount = {} WHERE id = {}
            ",
            update_entity.description.text,
            update_entity.roll.dice_label,
            update_entity.roll.dice,
            update_entity.roll.amount,
            entity_id
        );
        connection.execute(query).unwrap();
        Ok(())
    }
    fn delete(&self, database_path: &str, entity_id: u32) -> Result<(), String>
    {
        let connection = sqlite::Connection::open(database_path).unwrap();
        let query = format!(
            "
            DELETE FROM skills WHERE id = {};
            ",
            entity_id,
        );
        connection.execute(query).unwrap();
        Ok(())
    }
}

impl SaveLoad for Counter {
    type Entity = Counter;
    fn save(&self, database_path: &str, campaign_id: u32) -> Result<(), String>
    {
        let connection = sqlite::Connection::open(database_path).unwrap(); 
        let query = format!(
            "
            INSERT INTO counters (
            ttrpg_id,
            description,
            number
            )
            VALUES ({}, '{}', {})
            ",
            campaign_id,
            self.description.text,
            self.number
        );
        connection.execute(query).unwrap();

        Ok(())
    }
    fn update(&self, database_path: &str, entity_id: u32, update_entity: Self::Entity) -> Result<(), String>
    {
        let connection = sqlite::Connection::open(database_path).unwrap();
        let query = format!(
            "
            UPDATE counters SET description = '{}', number = {} WHERE id = {};
            ",
            update_entity.description.text,
            update_entity.number,
            entity_id
        );
        connection.execute(query).unwrap();
        Ok(())
    }
    fn delete(&self, database_path: &str, entity_id: u32) -> Result<(), String>
    {
        let connection = sqlite::Connection::open(database_path).unwrap();
        let query = format!(
            "
            DELETE FROM counters WHERE id = {}
            ",
            entity_id
        );
        connection.execute(query).unwrap();
        Ok(())
    }
}

impl SaveLoad for Table {
    type Entity = Table;
    fn save(&self, database_path: &str, campaign_id: u32) -> Result<(), String>
    {
        let connection = sqlite::Connection::open(database_path).unwrap(); 
        let query = format!(
            "
            INSERT INTO tables (
                ttrpg_id,
                description,
                values_json
            ) VALUES({}, '{}', '{}')
            ",
            campaign_id,
            self.description.text,
            self.table.values_to_json()
        );
        connection.execute(query).unwrap();
        Ok(())
    }
    fn update(&self, database_path: &str, entity_id: u32, update_entity: Self::Entity) -> Result<(), String>
    {
        let connection = sqlite::Connection::open(database_path).unwrap();
        let query_table = format!(
            "
            UPDATE tables SET description = '{}', values_json = '{}' WHERE id = {};
            ",
            update_entity.description.text,
            update_entity.table.values_to_json(),
            entity_id
        );
        connection.execute(query_table).unwrap();
        Ok(())
    }
    fn delete(&self, database_path: &str, entity_id: u32) -> Result<(), String>
    {
        let connection = sqlite::Connection::open(database_path).unwrap();
        let query = format!(
            "
            DELETE FROM tables WHERE id = {};
            ",
            entity_id
        );
        connection.execute(query).unwrap();
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ElementsEnum {
    Story(Story),
    Attribute(Attribute),
    Skill(Skill),
    Counter(Counter),
    Table(Table)
}
