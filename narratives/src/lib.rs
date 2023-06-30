use std::collections::{HashMap, hash_map::RandomState};
use serde::{Deserialize, Serialize};
use serde_json;
use roll_dice::*;
use pithy;

//literally raw, original text
#[derive(Clone, Debug)]
pub struct TypedNarrative {
    pub text: String,
}

impl TypedNarrative {
    pub fn new(text: String) -> TypedNarrative {
        TypedNarrative {
            text: escape_sql(text.as_str()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AutoNarrative {
    pub summary: HashMap<usize, pithy::Sentence, RandomState>,
}
impl AutoNarrative {
    pub fn new(text: TypedNarrative) -> AutoNarrative {
        let summary = AutoNarrative::summarize(text, 3, 100);
        
        AutoNarrative {
            summary
        }
    }
    //Will summarize and return all of the summarized sentences to which bias can be implemented
    fn summarize(text: TypedNarrative, min: usize, max: usize) -> HashMap<usize, pithy::Sentence> {
        let mut summary = pithy::Summariser::new();

        summary.add_raw_text("".to_string(), text.text.clone(), ",", min, max, false);
        summary.score_sentences_by_word_frequency(word_frequency(text.text.as_str()),3.0, 10.0);
        summary.sentences
        
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TabledNarratives {
    pub table: HashMap<(u32, u32), String>
}


impl TabledNarratives {
    // 2 values are kept as the key, in order to handle roll limits/ ranges. ie: if a roll is within
    // the range of 1..=10 print etc.
    pub fn new(table: Vec<((u32, u32), String)>) -> TabledNarratives {
        let hashmap: HashMap<(u32, u32), String> = table.into_iter().collect();
        TabledNarratives { table: hashmap }
    }
    // Serialize the table values into json to be easily stored in a database
    pub fn values_to_json(&self) -> String {
        let map = &self.table;
        let mut data = Vec::new();
        for (key, value) in map {
            let key_str = format!("{},{}", key.0, key.1);
            let item = serde_json::json!({key_str: value});
            data.push(item);
        }
        let value: serde_json::Value = data.into();
        serde_json::to_string(&value).unwrap()
    }
    // Use to Deserialize table values from it's Serialized json value
    pub fn values_from_json(json_str: &str) ->  HashMap<(u32, u32), String> {
        let value: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let mut map = HashMap::new();
        if let serde_json::Value::Array(items) = value {
            for item in items {
                if let serde_json::Value::Object(obj) = item {
                    for (key_str, value) in obj {
                        if let Ok((x, y)) = parse_key(&key_str) {
                            if let serde_json::Value::String(s) = value {
                                map.insert((x, y), s);
                            }
                        }
                    }
                }
            }
        }
        println!("{:#?}", &map);
        map
    }

    pub fn roll_to_text(&self, roll: &Outcome) -> String {
        let roll_result: u32 = roll.base_result;
        let mut result: String = String::from("");
        for (range, value) in self.table.iter() {
            if roll_result >= range.0 && roll_result <= range.1 {
                result = value.clone();
            }
        }
        if result.len() == 0 {
            panic!("Roll failed to produce a value.");
        }
        result
    }
}

fn parse_key(key_str: &str) -> Result<(u32, u32), std::num::ParseIntError> {
    let parts: Vec<&str> = key_str.split(',').collect();
    let x = parts[0].parse()?;
    let y = parts[1].parse()?;
    Ok((x, y))
}

fn word_frequency(text: &str) -> HashMap<String, f32> {
    let mut frequency = HashMap::new();
    let words = text.split_whitespace();
    let total_words = words.clone().count() as f32;
    for word in words {
        *frequency.entry(word.to_string()).or_insert(0f32) += 1f32;
    }
    for value in frequency.values_mut() {
        *value /= total_words;
    }
    frequency
}

fn escape_sql(input: &str) -> String {
    input.replace("'", "''")
}
