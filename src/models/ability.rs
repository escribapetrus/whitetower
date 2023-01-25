pub enum AbilityType {
    Physical,
    Magic,
    Hybrid,
}

pub struct Ability {
    name: String,
    category: AbilityType,
    raw_damage: u32,
    strength_multiplier: f32,
    magic_multiplier: f32,
}

impl Ability {
    pub fn new(
        name: &str,
        category: AbilityType,
        raw_damage: u32,
        strength_multiplier: f32,
        magic_multiplier: f32,
    ) -> Self {
        Self {
            name: name.to_string(),
            category,
            raw_damage,
            strength_multiplier,
            magic_multiplier,
        }
    }
}
