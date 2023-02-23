use std::collections::{HashMap, hash_map::RandomState};
use roll_dice::*;
use pithy;

trait Rolled {
    fn roll_to_text(&mut self, roll: &Roll) -> String;
}

// literally raw, original text
pub struct TypedNarrative {
    pub text: String,
}

impl TypedNarrative {
    pub fn new(text: String) -> TypedNarrative {
        TypedNarrative {
            text,
        }
    }
}

#[derive(Debug)]
pub struct AutoNarrative {
    pub summary: HashMap<usize, pithy::Sentence, RandomState>,
}
impl AutoNarrative {
    pub fn new(text: TypedNarrative) -> AutoNarrative {
        let summary = AutoNarrative::summarize(text, 100, 300);
        
        AutoNarrative {
            summary
        }
    }
    //Will summarize and return all of the summarized sentences to which bias can be implemented
    fn summarize(text: TypedNarrative, min: usize, max: usize) -> HashMap<usize, pithy::Sentence> {
        let mut summary = pithy::Summariser::new();
        summary.add_raw_text("".to_string(), text.text.clone(), ",", min, max, false);
        summary.sentences
        
    }
}


pub struct TabledNarratives;

