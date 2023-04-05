use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;

enum CharacterClass {
    Warrior,
    Mage,
    Thief,
}

struct Character {
    name: String,
    class: CharacterClass,
    hp: i32,
    mp: i32,
}

impl Character {
    fn new(name: &str, class: CharacterClass) -> Self {
        match class {
            CharacterClass::Warrior => Character {
                name: name.to_string(),
                class,
                hp: 100,
                mp: 20,
            },
            CharacterClass::Mage => Character {
                name: name.to_string(),
                class,
                hp: 60,
                mp: 80,
            },
            CharacterClass::Thief => Character {
                name: name.to_string(),
                class,
                hp: 80,
                mp: 50,
            },
        }
    }

    fn attack(&self) -> i32 {
        match self.class {
            CharacterClass::Warrior => 10,
            CharacterClass::Mage => 5,
            CharacterClass::Thief => 7,
        }
    }

    fn defend(&mut self, damage: i32) {
        self.hp -= damage;
    }
}

fn battle(player: Arc<Mutex<Character>>, enemy: Arc<Mutex<Character>>) -> Result<(), Box<dyn Error>> {
    loop {
        let damage = {
            let player = player.lock().unwrap();
            player.attack()
        };
        {
            let mut enemy = enemy.lock().unwrap();
            enemy.defend(damage);
            println!("{} deals {} damage to {}", player.lock().unwrap().name, damage, enemy.name);

            if enemy.hp <= 0 {
                println!("{} wins!", player.lock().unwrap().name);
                break;
            }
        }

        let damage = {
            let enemy = enemy.lock().unwrap();
            enemy.attack()
        };
        {
            let mut player = player.lock().unwrap();
            player.defend(damage);
            println!("{} deals {} damage to {}", enemy.lock().unwrap().name, damage, player.name);

            if player.hp <= 0 {
                println!("{} wins!", enemy.lock().unwrap().name);
                break;
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let player = Arc::new(Mutex::new(Character::new("Player", CharacterClass::Warrior)));
    let enemy = Arc::new(Mutex::new(Character::new("Enemy", CharacterClass::Thief)));

    let player_thread = thread::spawn(move || {
        battle(player.clone(), enemy.clone());
    });
    let enemy_thread = thread::spawn(move || {
        battle(enemy.clone(), player.clone());
    });

    player_thread.join().unwrap();
    enemy_thread.join().unwrap();

    Ok(())
}
