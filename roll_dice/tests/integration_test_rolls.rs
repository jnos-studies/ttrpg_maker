use roll_dice;

#[test]
fn outcome_returns_public_base_result_and_description_of_roll() {
    let critical = roll_dice::Critical::One;
    let roll = roll_dice::Roll::new(20,1);
    let outcome = roll_dice::Outcome::new(&roll, &critical, 0, false);

    println!("Base result: {}", outcome.base_result);
    println!("Roll description: {}", outcome.roll_description);

    let pub_base: Option<u32> = Some(outcome.base_result);
    let pub_description: Option<String> = Some(outcome.roll_description);

    assert!(
        pub_base.is_some() &&
        pub_description.is_some()
    );
}

#[test]
fn outcome_success_function_produces_tuple_of_winner_and_roll_difficulty() {
    let critical = roll_dice::Critical::One;
    let roll = roll_dice::Roll::new(20,1);
    let outcome = roll_dice::Outcome::new(&roll, &critical, 0, false);
    let opposing_outcome = roll_dice::Outcome::new(&roll, &critical, 0, false);

    let winner: Option<(bool, u32)> = Some(outcome.success_of_roll(&opposing_outcome, 15));
    
    assert!(winner.is_some() == true);
}
