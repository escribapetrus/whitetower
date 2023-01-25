use crate::models::fiend::Fiend;
use crate::models::hero::Hero;
// use crate::models::ability::Ability;
use crate::models::Character;

fn main() {}

fn run_menu() {
    for m in whitetower::MENU_ITEMS.into_iter().enumerate() {
        println!("{}{}", m.0, m.1);
    }
}
