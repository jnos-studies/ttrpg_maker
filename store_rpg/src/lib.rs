use std::fmt::format;
use sqlite;


pub fn save_entity<T: 'static>(entity: T, database_path: &str, campaign_id: &str) {
    let connection = sqlite::Connection::open(database_path).unwrap();
    
    //connection.execute("INSERT INTO ttrpgs(name) VALUES('test')").unwrap();
    
    //let mut ttrpgs = Vec::new();

    //connection.iterate("SELECT id, name, date FROM ttrpgs", |row| {
    //ttrpgs.push(
        //(
            //row[0].1.unwrap().to_string(),
            //row[1].1.unwrap().to_string(),
            //row[2].1.unwrap().to_string(),
        //)
    //);
    //true
    //}).unwrap();
    //
    //println!("{:#?}", ttrpgs);
}

pub fn database_setup(database_path: &str) { 
    let connection = sqlite::Connection::open(database_path).unwrap();

    connection.execute("CREATE TABLE ttrpgs (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        date DATETIME DEFAULT CURRENT_TIMESTAMP,
        name TEXT NOT NULL
    )").expect("Could not create initial table");
    
    connection.execute("CREATE TABLE stories (
        ttrpg_id INTEGER NOT NULL,
        label_text TEXT NOT NULL,
        text_data TEXT NOT NULL,
        FOREIGN KEY (ttrpg_id) REFERENCES ttrpgs(id)
    )").expect("Could not create table");
    
    connection.execute("CREATE TABLE attributes (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        ttrpg_id INTEGER NOT NULL,
        label_text TEXT NOT NULL,
        description TEXT NOT NULL,
        FOREIGN KEY (ttrpg_id) REFERENCES ttrpgs(id)
    )").expect("Could not create table");

    connection.execute("CREATE TABLE attribute_outcomes (
        attribute_id INTEGER NOT NULL,
        roll_description TEXT NOT NULL,
        base_result INTEGER NOT NULL,
        FOREIGN KEY (attribute_id) REFERENCES attributes(id)
    )").expect("Could not create table");
    
    connection.execute("CREATE TABLE rolls (
        ttrpg_id INTEGER NOT NULL,
        skill_id INTEGER AUTOINCREMENT,
        blank_roll INTEGER NOT NULL,
        dice_label TEXT NOT NULL,
        dice INTEGER NOT NULL,
        amount INTEGER NOT NULL,
        FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id)
    )").expect("Could not create table");

    connection.execute("CREATE TABLE skills (
        ttrpg_id INTEGER NOT NULL,
        roll_id INTEGER NOT NULL,
        label_text TEXT NOT NULL,
        description TEXT NOT NULL,
        FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id),
        FOREIGN KEY (roll_id) REFERENCES rolls(skill_id)
    );").expect("Could not create table");
  
    connection.execute("CREATE TABLE counters (
        ttrpg_id INTEGER NOT NULL,
        label_text TEXT NOT NULL,
        description TEXT NOT NULL,
        number INTEGER NOT NULL,
        FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id)
    )").expect("Could not create table");
  
    connection.execute("CREATE TABLE tables (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        ttrpg_id INTEGER NOT NULL,
        label_text TEXT NOT NULL,
        description TEXT,
        FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id)
    )").expect("Could not create table");
  
    connection.execute("CREATE TABLE table_values (
        table_id INTEGER NOT NULL,
        lower_range INTEGER NOT NULL,
        higher_range INTEGER NOT NULL,
        text_value TEXT NOT NULL,
        FOREIGN KEY (table_id) REFERENCES tables(id)
    )").expect("Could not create table");
}
