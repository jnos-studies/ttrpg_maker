use store_rpg::database_setup;
use entities::*;
use narratives::*;
use sqlite;
use libtext;
use regex::Regex;

fn main () {
    libtext::record_audio("test_wavs/test3.wav").unwrap();
    let text = libtext::transcribe_audio_file("test_wavs/test3.wav");
    //need a single quote for type narratives
    
    let format_for_sql = format!("{}", text.text);
    let format_for_sql = escape_sql(&format_for_sql.as_str());
    database_setup("saves/test.db"); 
    println!("{}", format_for_sql);
    let text = TypedNarrative::new(format_for_sql.to_string());
    
    let story = Story::new(text);
    story.save("saves/test.db",1).expect("YOU FAILED");
    story.save("saves/test.db", 2).expect("YOU FAILED YET AGAIN!");
    let connection = sqlite::Connection::open("saves/test.db").unwrap();

    
    connection.iterate("SELECT * FROM stories", |row| {
        println!("{:?}", row);
        true
    }).unwrap();
}

fn escape_sql(input: &str) -> String {
    let re = Regex::new(r#"([\\'"])"#).unwrap();
    let escaped = re.replace_all(input, "");
    escaped.to_string()
}
