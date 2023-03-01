use roll_dice::*;
use narratives::*;

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

impl Skill {
    pub fn new(description: TypedNarrative, roll: Roll) -> Skill {
        Skill {
            description,
            roll
        }
    }
}

// Counter
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

}
// Table
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


