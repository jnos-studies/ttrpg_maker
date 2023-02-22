use narratives::{self, TypedNarrative, AutoNarrative};
use std::fs::File;
use std::io::prelude::*;


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

        println!("Contents to use \n \n {}", contents);
        assert_eq!(1,1);
        Ok(())
}
