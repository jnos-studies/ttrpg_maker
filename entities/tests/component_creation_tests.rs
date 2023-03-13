use narratives::*;
use entities::*;
use roll_dice::{Roll, Outcome, Critical};

const LABEL: &str = "Test_label";

#[test]
fn story_component_created_with_raw_and_summarized_text() {
    let raw_text = TypedNarrative::new(
            "To create new stories, or components which simply contain text data, you
            create a Story component. You use it for things such as simple descriptions
            of a physical area, a plot point, a character description etc. You can also
            use story components when storing transcribed audio into a data struct. Each
            new piece of text could be stored as a Story data struct to be honest, but
            then it kind of loses the point of creating this whole thing.".to_string()
    );

    let story_component = Story::new(raw_text, &LABEL);
    
    let summary = &story_component
        .summarized.summary
        .values()
        .last()
        .unwrap()
        .text;
    
    assert!(
        story_component.raw_narration.len() > 0 &&
        summary.len() > 0
    );
}


#[test]
fn attribute_component_created_with_outcome() {
    let test_attribute_string = TypedNarrative::new(String::from("This is an attribute"));
    let roll = Roll::new(6, 4);
    let crit = Critical::One;
    // Bool parameter true means that the outcome is associated with an attribute but not
    // necesarily a part of an attribute component
    let outcome = Outcome::new(&roll, &crit, 0, true);

    let attribute = Attribute::new(test_attribute_string, outcome, &LABEL);

    // test the attribute outcome against opposing roll, ie opposing roll between the same skills
    let opposing_roll = Outcome::new(&roll, &crit, 0, true);
    let difficulty = 10;
    let result = Some(attribute.attribute.success_of_roll(&opposing_roll, difficulty));

    assert!(
        attribute.attribute.base_result > 0 &&
        attribute.description.text.len() > 0 &&
        result.is_some()
    );
}


#[test]
fn skill_created_and_returns_a_roll_and_description() {
    let description = TypedNarrative::new(String::from("This is a skill"));
    let roll = Roll::new(4, 1);
    let skill = Skill::new(description, roll, &LABEL);
    let skill_description = Some(&skill.description.text);
    let skill_roll = Some(&skill.roll);
    
    assert!(
        skill_description.is_some() &&
        skill_roll.is_some()
    );
}


#[test]
fn counter_created_and_returns_description_and_number_that_can_change() {
    let counter_description = TypedNarrative::new(String::from("This is a Counter"));
    let mut counter = Counter::new(counter_description, 4, &LABEL);

    counter.change_number(2);

    assert!(
        counter.description.text.len() > 0 &&
        counter.number != 4
    );
}


#[test]
fn create_table_that_returns_table_with_description(){
    let table_description = TypedNarrative::new(String::from("This is a table"));
    let table_values = vec![
        ((1, 1),String::from("One")),
        ((2, 2), String::from("Two")),
        ((3, 3), String::from("Three"))
    ];

    let table_narrative = TabledNarratives::new(table_values);
    let table = Table::new(table_description, table_narrative, &LABEL);

    let roll = Roll::new(3, 1);
    let crit = Critical::One;
    let outcome = Outcome::new(&roll, &crit, 0, false);

    let rolled_text = Some(table.table.roll_to_text(&outcome));

    assert!(
        table.description.text.len() > 0 &&
        rolled_text.is_some()
    );

}










