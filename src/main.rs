pub mod models;

use crate::models::fiend::Fiend;
use crate::models::hero::Hero;
// use crate::models::ability::Ability;
use crate::models::Character;

fn main() {
    let mc: Hero = Hero::new("Rina Sawayama".to_string(), 1000, 10);
    let enemy: Fiend = Fiend::new("Red Demon".to_string(), 10000, 10);

    println!("Main character: {}", mc.get_name());
    println!("Enemy: {}", enemy.get_name());
}

fn run_menu() {
    for m in whitetower::MENU_ITEMS.into_iter().enumerate() {
        println!("{}{}", m.0, m.1);
    }
}
