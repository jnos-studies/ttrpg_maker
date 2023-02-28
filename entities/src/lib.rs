use roll_dice::*;
use narratives::*;

// 
pub trait Entity {
    fn create(&self);
}

pub struct Interface {
    pub components: Vec<Box<dyn Entity>>
}

impl Interface {
    // create is implemented elsewhere when defining all of the components that make up a
    // single interface
    fn create(&self) {
        for component in self.components.iter() {
            component.create();
        }
    }
}

// Public structs that are components to the Interface
// A single interface could represent a character sheet, an NPC etc.

// Story
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
pub struct Attribute {
    pub description: TypedNarrative,
    pub attribute: Outcome,
}

impl Attribute {
    pub fn new(description: TypedNarrative, attribute: Outcome) -> Attribute {
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
pub struct Skill {
    pub description: TypedNarrative,
    pub roll: Roll
}

// Counter
pub struct Counter {
    pub description: TypedNarrative,
    pub number: u32
}

// Table
pub struct Table {
    pub description: TypedNarrative,
    pub table: TabledNarratives
}


