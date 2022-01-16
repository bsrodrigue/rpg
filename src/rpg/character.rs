use crate::rpg::potion::Potion;
use crate::Attack;
use std::fmt;

pub struct Character {
    pub name: String,
    pub life_points: u64,
    pub attacks: Vec<Attack>,
}

impl Character {
    pub fn drink_potion(&mut self, potion: &Potion) {
        self.life_points += potion.value;
    }

    pub fn learn_attack(&mut self, attack: Attack) {
        let attack_name = String::from(&attack.name);
        self.attacks.push(attack);
        println!("{} learned {}", self.name, attack_name);
    }

    pub fn get_attack_str(&self) -> String {
        let mut output = String::new();

        for attack in &self.attacks {
            output += format!("{}, ", attack).as_str();
        }

        output
    }

    // fn attack_character(&mut self, target: &mut Character) {
    //     target.life_points -= self.attack;
    // }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
