pub mod ability;
pub mod fiend;
pub mod hero;

pub trait Character {
    fn get_name(&self) -> &String;
    fn is_alive(&self) -> bool;
    fn is_ready(&self) -> bool;
    fn get_life_total(&self) -> u32;
    fn get_life(&self) -> u32;
    fn get_initiative(&self) -> u32;
    fn get_speed(&self) -> u32;
    fn decrease_life_by(&mut self, x: u32);
    fn increase_life_by(&mut self, x: u32);
    fn restore_life(&mut self);
    fn restore_initiative(&mut self);
    fn reset_initiative(&mut self);
    fn increase_initiative(&mut self);
}
