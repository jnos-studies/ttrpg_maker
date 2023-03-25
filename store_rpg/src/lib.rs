
pub fn database_setup(database_path: &str) { 
    let connection = sqlite::Connection::open(database_path).unwrap();

    let query = "CREATE TABLE ttrpgs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date DATETIME DEFAULT CURRENT_TIMESTAMP,
            name TEXT NOT NULL);

        CREATE TABLE stories (
            ttrpg_id INTEGER NOT NULL,
            text_data TEXT NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpgs(id)
        );
    
        CREATE TABLE attributes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ttrpg_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpgs(id)
        );

        CREATE TABLE attribute_outcomes (
            attribute_id INTEGER NOT NULL,
            roll_description TEXT NOT NULL,
            base_result INTEGER NOT NULL,
            FOREIGN KEY (attribute_id) REFERENCES attributes(id)
        );
    
        CREATE TABLE rolls (
            ttrpg_id INTEGER NOT NULL,
            skill_id INTEGER PRIMARY KEY AUTOINCREMENT,
            blank_roll INTEGER NOT NULL,
            dice_label TEXT NOT NULL,
            dice INTEGER NOT NULL,
            amount INTEGER NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id)
        );

        CREATE TABLE skills (
            ttrpg_id INTEGER NOT NULL,
            roll_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id),
            FOREIGN KEY (roll_id) REFERENCES rolls(skill_id)
        );
  
        CREATE TABLE counters (
            ttrpg_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            number INTEGER NOT NULL,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id)
        );
  
        CREATE TABLE tables (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ttrpg_id INTEGER NOT NULL,
            description TEXT,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id)
        );
  
        CREATE TABLE table_values (
            table_id INTEGER NOT NULL,
            lower_range INTEGER NOT NULL,
            higher_range INTEGER NOT NULL,
            text_value TEXT NOT NULL,
            FOREIGN KEY (table_id) REFERENCES tables(id)
        );
    ";

    connection.execute(query).unwrap();
}
