use crate::models::Character;

const READY: u32 = 200;

pub struct Fiend {
    name: String,
    life_total: u32,
    life: u32,
    speed: u32,
    initiative: u32,
    strength: u32,
    magic: u32,
}

impl Fiend {
    pub fn new(name: String, life_total: u32, speed: u32) -> Self {
        Self {
            name,
            life_total,
            speed,
            life: life_total,
            initiative: READY,
            strength: 10,
            magic: 10,
        }
    }
}

impl Character for Fiend {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn is_alive(&self) -> bool {
        self.life > 0
    }
    fn is_ready(&self) -> bool {
        self.initiative == READY
    }
    fn get_life_total(&self) -> u32 {
        self.life_total
    }
    fn get_life(&self) -> u32 {
        self.life
    }
    fn get_initiative(&self) -> u32 {
        self.initiative
    }
    fn get_speed(&self) -> u32 {
        self.speed
    }
    fn decrease_life_by(&mut self, x: u32) {
        if x > self.life {
            self.life = 0
        } else {
            self.life -= x
        }
    }
    fn increase_life_by(&mut self, x: u32) {
        let incr = self.life + x;
        if incr < self.life_total {
            self.life = incr
        } else {
            self.life = self.life_total
        }
    }
    fn restore_life(&mut self) {
        self.life = self.life_total
    }
    fn restore_initiative(&mut self) {
        self.initiative = READY
    }
    fn reset_initiative(&mut self) {
        self.initiative = 0
    }
    fn increase_initiative(&mut self) {
        let incr = self.initiative + self.speed;
        if incr < READY {
            self.initiative = incr
        } else {
            self.initiative = READY
        }
    }
}
