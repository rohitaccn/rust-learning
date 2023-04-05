use std::error::Error;

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

fn main() -> Result<(), Box<dyn Error>> {
    let mut player = Character::new("Player", CharacterClass::Warrior);
    let mut enemy = Character::new("Enemy", CharacterClass::Thief);

    loop {
        let damage = player.attack();
        enemy.defend(damage);
        println!("{} deals {} damage to {}", player.name, damage, enemy.name);

        if enemy.hp <= 0 {
            println!("{} wins!", player.name);
            break;
        }

        let damage = enemy.attack();
        player.defend(damage);
        println!("{} deals {} damage to {}", enemy.name, damage, player.name);

        if player.hp <= 0 {
            println!("{} wins!", enemy.name);
            break;
        }
    }

    Ok(())
}
