pub mod engine;
pub mod models;

use crate::models::ability::Ability;
use crate::models::fiend::Fiend;
use crate::models::hero::Hero;
use crate::models::Character;

pub const MENU_ITEMS: [&str; 5] = [
    "Party status",
    "Go hunt",
    "Do quest",
    "Create abilities",
    "Buy equipment",
];

fn run_menu() {
    for m in MENU_ITEMS.into_iter().enumerate() {
        println!("{}{}", m.0, m.1);
    }
}
