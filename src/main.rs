mod rpg;
use rpg::{abilities::attack::Attack, entities::character::Character, objects::potion::Potion};

fn main() {
    let potion = Potion { value: 50 };
    let fire_breath = Attack {
        name: String::from("Fire Breath"),
        damage: 500,
    };
    let mut rodrigue = Character {
        name: String::from("Rodrigue"),
        life_points: 500,
        attacks: vec![],
    };

    rodrigue.drink_potion(&potion);

    println!(
        "Here are the attacks that {} knows before learning fire breath: {}",
        rodrigue,
        rodrigue.get_attack_str()
    );

    rodrigue.learn_attack(fire_breath);

    println!(
        "Here are the attacks that {} knows after learning fire breath: {}",
        rodrigue,
        rodrigue.get_attack_str()
    );
}
