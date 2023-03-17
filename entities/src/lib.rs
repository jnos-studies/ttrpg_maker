use roll_dice::*;
use narratives::*;


// Story
pub struct Story<'a> {
    pub label_id: &'a str,
    pub raw_narration: String,
    pub summarized: AutoNarrative
}

impl Story<'_> {
     pub fn new<'a>(raw_narration: TypedNarrative, label_id: &'a str) -> Story<'a> {
         Story {
            label_id,
            raw_narration: raw_narration.text.to_owned(),
            summarized: AutoNarrative::new(raw_narration)
        }
     }
}

// Attribute
pub struct Attribute<'a> {
    pub label_id: &'a str,
    pub description: TypedNarrative,
    pub attribute: Outcome,
}

impl Attribute<'_> {
    pub fn new<'a>(description: TypedNarrative, attribute: Outcome, label_id: &'a str) -> Attribute<'a> {
        if attribute.attribute {
            Attribute {
                label_id,
                description,
                attribute
            }
        } else {
            panic!("Unable to create attribute: attribute set to false")
        }
    }
}


// Skill
pub struct Skill<'a> {
    pub label_id: &'a str,
    pub description: TypedNarrative,
    pub roll: Roll
}

impl Skill<'_> {
    pub fn new<'a>(description: TypedNarrative, roll: Roll, label_id: &'a str) -> Skill<'a> {
        Skill {
            label_id,
            description,
            roll
        }
    }
}

// Counter
pub struct Counter<'a> {
    pub label_id: &'a str,
    pub description: TypedNarrative,
    pub number: u32
}

impl Counter<'_> {
    pub fn new<'a>(description: TypedNarrative, number: u32, label_id: &'a str) -> Counter<'a> {
        Counter {
            label_id,
            description,
            number
        }
    }

    pub fn change_number(&mut self, new_number: u32) {
        self.number = new_number;
    }
}
// Table
pub struct Table<'a> {
    pub label_id: &'a str,
    pub description: TypedNarrative,
    pub table: TabledNarratives
}

impl Table<'_> {
    pub fn new<'a>(description: TypedNarrative, table: TabledNarratives, label_id: &'a str) -> Table<'a> {
        Table {
            label_id,
            description,
            table
        }
    }
}
