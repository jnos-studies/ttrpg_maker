#![allow(dead_code)] // allow dead code. comment out for debugging purposes
use rand::{thread_rng, Rng};

trait DiceRoll {
    fn roll(&self) -> (Vec<u32>, String);
}

#[derive(Debug)]
pub struct Roll {
    pub dice_label: String,
    pub dice: u32,
    pub amount: u32
}

impl Roll {
    pub fn new(dice: u32, amount: u32) -> Roll {
        let label = format!("{}d{}", &amount, &dice).to_string();
        Roll {
            dice_label: label,
            dice,
            amount
        }
    }
}

impl DiceRoll for Roll {
    fn roll(&self) -> (Vec<u32>, String) {
        let mut rng = thread_rng();
        let mut rolls: Vec<u32> = vec![];

        for _ in 0..self.amount {
            rolls.push(rng.gen_range(1..= self.dice));
        }

        (rolls, self.dice_label.clone())
    }
}
//The Outcome struct handles opposed rolls and determines who wins depending on
//the critical_value. The critical value determines whether the highest die is 1 or 20. 

pub enum Critical {
    Twenty,
    One
}


#[derive(Debug)]
pub struct Outcome {
    pub roll_description: String,
    pub base_result: u32,
    max: u32,
    min: u32,
    pub attribute: bool,
    critical: u32
}
    
impl Outcome {
    pub fn new(roll: &Roll, crit: &Critical, bonus: u32, attribute: bool) -> Outcome {
        let critical: u32 = match crit {
            Critical::Twenty => 20,
            Critical::One => 1,
        };
        
        let mut rolled = roll.roll().0;
               
        let roll_description = format!("Roll: {} + {}", roll.dice_label, &bonus);

        let (max, min) =
            if critical == 20 {
                (*rolled.iter().max().unwrap(), *rolled.iter().min().unwrap())
            }
            else {
                (*rolled.iter().min().unwrap(),  *rolled.iter().max().unwrap())
            };

        //change this so if attribute is true, it removes the lowest dice roll before summation
        //future implementations might include other systems of attribute selection, ie: other
        //rules. This attribute picker is based on Dragonbane rules
        if attribute == true {
            if let Some(min_index) = rolled.iter().position(|&x| x == min) {
                rolled.remove(min_index);
            }
        }

        let mut  base_result: u32 = rolled.iter().sum();
        base_result += bonus;
        
        Outcome {
            roll_description,
            base_result,
            max,
            min,
            attribute,
            critical
        }
    }

    //compares values of the opposed roller and their opposition. Used for circumstances where
    //the player rolls against a monster (their opposition). The difficulty paramater is used as
    //an override if needed
    pub fn success_of_roll(&self, opposition: &Outcome, difficulty: u32) -> (bool, u32) {
        let difficulty = if opposition.attribute == true
                        {opposition.base_result.clone()} else {difficulty};
        
        let winner: bool; //shadow variable for the if else statement
        
        if self.critical == 20 {
            winner = if self.base_result >= difficulty && self.base_result >= opposition.base_result 
                        {true} else {false}
        }
        else {
            winner = if self.base_result <= difficulty && self.base_result <= opposition.base_result
                        {true} else {false}
        }

        (winner, difficulty)
    }
}










