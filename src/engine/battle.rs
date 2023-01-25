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
fn surviving<T: Character> (chars: Vec<T>) -> usize{
    let rem: Vec<T> = chars
	.into_iter()
	.filter(|c| c.is_alive())
	.collect();

    rem.len()
}

fn active<T: Character> (chars: Vec<T>) -> T {
    let reducer =  |c: T, acc: T| {
	if c.get_speed() > acc.get_speed() {c}
	else {acc}
    };
    let rem: Vec<T> = chars
        .into_iter()
        .filter(|c| c.is_ready())
        .fold(reducer)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_surviving_characters() {
	
    }
}
