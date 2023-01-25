use crate::models::fiend::Fiend;
use crate::models::hero::Hero;
use crate::models::Character;


enum BattleResult {
    Victory,
    Defeat,
    Ongoing,
}

// fn battle(party: Vec<Hero>, enemies: Vec<Fiend>) {

//     // println!("Main character: {}", mc.get_name());
//     // println!("Enemy: {}", enemy.get_name());
// }
fn surviving<T: Character>(chars: Vec<T>) -> usize {
    chars
        .into_iter()
        .filter(|c| c.is_alive())
        .collect::<Vec<T>>()
        .len()
}

fn active<T: Character>(chars: Vec<T>) -> T {
    let reducer = |c: T, acc: T| {
        if c.get_speed() > acc.get_speed() {
            c
        } else {
            acc
        }
    };
    chars
        .into_iter()
        .filter(|c| c.is_ready())
        .reduce(reducer)
        .expect("Error reducing iter")
}

fn update_initiative<T: Character>(chars: Vec<T>){
    for mut c in chars {
	c.increase_initiative();
    }
}

// fn attack()

    //     action: (abilityIndex, source, target) => {
    //     let { type, amount } = source.cast(abilityIndex);
        
    //     if (type == "damage") target.decreaseLifeBy(amount);
    //     source.resetInitiative();
    // },

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_active_character() {
        let c1 = Hero::new("Rina Sawayama".to_string(), 1000, 10);
        let c2 = Hero::new("Second Character".to_string(), 2000, 20);
        let c3 = Hero::new("Third Character".to_string(), 3000, 5);
        let party = vec![c1, c2, c3];

        let act = surviving(party);
        dbg!(act);
        assert_eq!(act, 20);
    }
}
