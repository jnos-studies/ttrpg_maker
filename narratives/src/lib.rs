use roll_dice::*;
use pithy;

trait AutoText {
    fn summarize(&mut self, text: &mut TypedNarrative) -> Option<pithy::Sentence>;
}

trait Rolled {
    fn roll_to_text(roll: &Roll) -> String;
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

pub struct AutoNarrative {
    pub summary: String,
}

impl AutoNarrative {
    pub fn new(&mut self, text: &mut TypedNarrative) -> AutoNarrative {

        let summary = self.summarize(text);
        
        match summary {
            Some(s) => AutoNarrative { summary: s.text},
            None => AutoNarrative { summary:  "no summary available".to_string()}
        }
    }
}
impl AutoText for AutoNarrative {
    fn summarize(&mut self, text: &mut TypedNarrative) -> Option<pithy::Sentence> {
        let mut summary = pithy::Summariser::new();
        summary.add_raw_text("../n.txt".to_string(), text.text.clone(), ",",30, 500, false);
        let result = summary.retrieve_sentence_by_index(0);

        result
        
    }
}


pub struct TabledNarratives;

