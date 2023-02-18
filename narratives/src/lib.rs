use roll_dice::*;

trait AutoText {
    fn summarize(&mut self, text: &mut TypedNarrative) -> String;
}

trait Rolled {
    fn roll_to_text(roll: &Roll) -> String;
}

// literally raw, original text
pub struct TypedNarrative {
    text: String,
}

impl TypedNarrative {
    pub fn new(text: String) -> TypedNarrative {
        TypedNarrative {
            text,
        }
    }
}

pub struct AutoNarrative {
    summary: String,
}

impl AutoNarrative {
    pub fn new(&mut self, text: &mut TypedNarrative) -> AutoNarrative {
        let summary = self.summarize(text);
        AutoNarrative {
            summary
        }
    }
}
impl AutoText for AutoNarrative {
    fn summarize(&mut self, text: &mut TypedNarrative) -> String {
        let summary = text.text.clone(); // later actually summarize. Functionality is not implemented yet
        summary
    }
}


pub struct TabledNarratives;

