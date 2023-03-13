use sqlite;
//use entities::*;
use std::path::Path;
pub fn database_setup(database_path: &str) {
    let database_path = Path::new(database_path);
    let connection = sqlite::open(database_path).unwrap();
    let query = "
        CREATE TABLE ttrpgs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date DATETIME DEFAULT CURRENT_TIMESTAMP,
            name TEXT NOT NULL
        );
        CREATE TABLE stories (
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpgs(id),
            label TEXT NOT NULL,
            text_data TEXT NOT NULL
        );
        CREATE TABLE attributes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpgs(id),
            label TEXT NOT NULL,
            description TEXT NOT NULL,
            outcome FOREIGN KEY (outcome_id) REFERENCES attribute_outcomes(attribute_id)
        );
        CREATE TABLE attribute_outcomes (
            FOREIGN KEY (attribute_id) REFERENCES attributes(id),
            roll_description TEXT NOT NULL,
            base_result INTEGER NOT NULL,
            max INTEGER NOT NULL,
            min INTEGER NOT NULL
        );
       CREATE TABLE skills (
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id),
            label TEXT NOT NULL,
            description TEXT NOT NULL,
            roll FOREIGN KEY (roll_id) REFERENCES rolls(skill_id)
       );

       CREATE TABLE rolls (
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id),
            skill_id INTEGER,
            blank_roll INTEGER NOT NULL,
            dice_label TEXT NOT NULL,
            dice INTEGER NOT NULL,
            amount INTEGER NOT NULL
       );
       
       CREATE TABLE counters (
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id),
            label TEXT NOT NULL,
            description TEXT NOT NULL,
            number INTEGER NOT NULL
       );

       CREATE TABLE tables (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            FOREIGN KEY (ttrpg_id) REFERENCES ttrpg(id),
            label TEXT NOT NULL,
            description TEXT,
       );
       
       CREATE TABLE table_values (
            FOREIGN KEY (table_id) REFERENCES tables(id),
            lower_range INTEGER NOT NULL,
            higher_range INTEGER NOT NULL,
            text_value TEXT NOT NULL
       );
    ";
    connection.execute(query).unwrap();
}
