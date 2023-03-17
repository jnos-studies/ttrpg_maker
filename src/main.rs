use egui::TextBuffer;
use store_rpg::{save_entity, database_setup};
use entities::*;

fn main () {
    //let text = TypedNarrative::new("Hello world");
    //let story = Story::new(text, "test");
    //println!("{:#?}", story);
    //save_entity(story, "saves/test.db", "1".as_str());
    database_setup("saves/test.db");
}
