use narratives::{self, TypedNarrative, AutoNarrative, TabledNarratives};
use std::fs::File;
use std::io::prelude::*;
use roll_dice::*;


#[test]
fn can_create_and_retrive_texts() {
    let text = String::from("This is a String");
    let typed_narrative = TypedNarrative::new(text.clone());
    let tn_text = &typed_narrative.text;

    println!("Text: {}", tn_text);

    let tn_result: Option<TypedNarrative> = Some(TypedNarrative { text });

    let get_text_result: Option<&String> = Some(tn_text);

    assert!(tn_result.is_some() && get_text_result.is_some());

}

#[test]
fn can_create_summaries_of_large_text_using_pithy() -> std::io::Result<()>{
        let mut file = File::open("samples.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let typed_narrative = Some(TypedNarrative::new(contents.clone()));
        let auto_narrative = Some(AutoNarrative::new(TypedNarrative::new(contents.clone())));
        
        //println!("\n\nSummary \n\n {:#?}", auto_narrative);
        
        assert!(
            typed_narrative.is_some() &&
            auto_narrative.is_some()
            );

        Ok(())
}

#[test]
fn can_create_tables_and_retrieve_text_from_roll() {
    let table_values = vec![
        ((1, 5), String::from("You eat dirt")),
        ((6, 10), String::from("You lost all your money!")),
        ((11, 15), String::from("After falling, you broke a leg!")),
        ((16, 20), String::from("Turns out the child was in fact yours!")),
        ((21, 25), String::from("Your rampat alcoholism causes eternal poisoning!")),
        ((26, 40), String::from("You ran out of things to say..."))
    ];

    let test_table = TabledNarratives::new(table_values);
    let roll = Roll::new(20,2);
    let crit = Critical::One;
    let outcome = Outcome::new(&roll, &crit, 0, false);
    let result_pass = test_table.roll_to_text(&outcome);

    assert!(Some(result_pass).is_some());
}

#[test]
#[should_panic]
fn a_roll_that_is_out_of_range_of_a_table_will_panic() {
    let table_values = vec![
        ((1, 5), String::from("You eat dirt")),
        ((6, 10), String::from("You lost all your money!")),
        ((11, 15), String::from("After falling, you broke a leg!")),
        ((16, 20), String::from("Turns out the child was in fact yours!")),
        ((21, 25), String::from("Your rampat alcoholism causes eternal poisoning!")),
        ((26, 40), String::from("You ran out of things to say..."))
    ];

    let test_table = TabledNarratives::new(table_values);

    let roll = Roll::new(20,100);
    let crit = Critical::One;
    let outcome = Outcome::new(&roll, &crit, 0, false);
    test_table.roll_to_text(&outcome);
}
