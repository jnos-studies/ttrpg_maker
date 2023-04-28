use roll_dice::*;
use narratives::*;
use sqlite;

// Story
#[derive(Debug)]
pub struct Story {
    pub raw_narration: String,
    pub summarized: AutoNarrative
}

impl Story {
     pub fn new(raw_narration: TypedNarrative) -> Story {
         Story {
            raw_narration: raw_narration.text.to_owned(),
            summarized: AutoNarrative::new(raw_narration)
        }
     }
}

// Attribute
#[derive(Clone)]
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
#[derive(Clone)]
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
#[derive(Clone)]
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
#[derive(Clone)]
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

// Implement save method for all entity objects
pub trait SaveLoad {
    type Entity;
    fn save(&self, database_path: &str, campaign_id: u32) -> Result<(), String>;
    fn update(&self, database_path: &str, entity_id: u32, update_entity: Self::Entity) -> Result<(), String>;
}

impl SaveLoad for Story {
    type Entity = Story;
    fn save(&self, database_path: &str, campaign_id: u32) -> Result<(), String> {
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
            "UPDATE stories SET text_data = '{}' WHERE id = {};",
            update_entity.raw_narration,
            entity_id
        );

        connection.execute(query).unwrap();
        Ok(())
    }
}

impl SaveLoad for Attribute {
    type Entity = Attribute;
    fn save(&self, database_path: &str, campaign_id: u32) -> Result<(), String> {
        let connection = sqlite::Connection::open(database_path).unwrap(); 
        let query_attribute = format!(
            "
            INSERT INTO attributes (
                ttrpg_id,
                description
            )
            VALUES ({}, '{}')
            ",
            campaign_id,
            self.description.text
        );
        connection.execute(query_attribute).unwrap();

        let mut attribute_id = 0;
        connection.iterate("SELECT * FROM attributes", |_| {
            attribute_id += 1;
            true
        }).unwrap();

        let query_attribute_outcome = format!(
            "
                INSERT INTO attribute_outcomes (
                    attribute_id,
                    roll_description,
                    base_result
                )
                VALUES ({}, '{}', {})
            ",
            attribute_id,
            self.attribute.roll_description,
            self.attribute.base_result
        );
        connection.execute(query_attribute_outcome).unwrap();
        Ok(())
    }

    fn update(&self, database_path: &str, entity_id: u32, update_entity: Self::Entity) -> Result<(), String>
    {
        let connection = sqlite::Connection::open(database_path).unwrap();
        // Remove and add a new attribute to replace the old one
        let query = format!(
            "
            UPDATE attribute_outcomes SET roll_description = '{}', base_result = {} WHERE attribute_id = {};
            UPDATE attributes SET description WHERE id = {};
            ",
            update_entity.attribute.roll_description,
            update_entity.attribute.base_result,
            entity_id,
            entity_id
        );

        connection.execute(query).unwrap();
        Ok(())
        
    }
}

impl SaveLoad for Skill {
    type Entity = Skill;
    fn save(&self, database_path: &str, campaign_id: u32) -> Result<(), String> {
        let connection = sqlite::Connection::open(database_path).unwrap(); 
        let query_roll = format!(
            "
                INSERT INTO rolls (
                    ttrpg_id,
                    blank_roll,
                    dice_label,
                    dice,
                    amount
                ) VALUES ({}, {}, '{}', {}, {})
            ",
            campaign_id,
            0,
            self.roll.dice_label,
            self.roll.dice,
            self.roll.amount
        );
        connection.execute(query_roll).unwrap();
        
        let mut roll_id = 0;
        connection.iterate("SELECT *  FROM skills", |_| {
            roll_id += 1;
            true
        }).unwrap();

        let query_skill = format!(
            "
            INSERT INTO skills (
                ttrpg_id,
                roll_id,
                description
            )
            VALUES ({}, {}, '{}')
            ",
            campaign_id,
            roll_id,
            self.description.text
        );
        connection.execute(query_skill).unwrap();

        Ok(())
    }
    fn update(&self, database_path: &str, entity_id: u32, update_entity: Self::Entity) -> Result<(), String> {
        Ok(())
    }
}

impl SaveLoad for Counter {
    type Entity = Counter;
    fn save(&self, database_path: &str, campaign_id: u32) -> Result<(), String> {
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

    fn update(&self, database_path: &str, entity_id: u32, update_entity: Self::Entity) -> Result<(), String> {
        Ok(())
    }
}

impl SaveLoad for Table {
    type Entity = Table;
    fn save(&self, database_path: &str, campaign_id: u32) -> Result<(), String> {
        let connection = sqlite::Connection::open(database_path).unwrap(); 
        let query_table = format!(
            "
            INSERT INTO tables (
                ttrpg_id,
                description
            )
            VALUES ({}, '{}')
            ",
            campaign_id,
            self.description.text
        );
        connection.execute(query_table).unwrap();

        let mut table_id = 0;
        connection.iterate("SELECT * FROM tables", |_| {
            table_id += 1;
            true
        }).unwrap();
        
        for (key, value) in self.table.table.iter(){
            let query_table_values = format!(
                "
                    INSERT INTO table_values (
                        table_id,
                        lower_range,
                        higher_range,
                        text_value
                    )
                    VALUES ({}, {}, {}, '{}')
                ",
                table_id,
                key.0, // lower_range
                key.1, // higher_range
                value  //text_value
            );
            connection.execute(query_table_values).unwrap();
        }
        
        Ok(())
    }

    fn update(&self, database_path: &str, entity_id: u32, update_entity: Self::Entity) -> Result<(), String> {
        Ok(())
    }
}
