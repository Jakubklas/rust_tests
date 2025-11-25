
struct Player {
    name: String,
    hp: i32,
    hunger: i32,
}

impl Player {
    fn new(name: &str) -> Player{
        Player {
            name: String::from(name),
            hp: 100,
            hunger: 0
        }
    }

    fn get_hungry(&mut self) {
        self.hunger += 10;
        if self.hunger >=100 {
            println!("DEAD!!! - Starved to death.");
            return;
        }
    }

    fn eat(&mut self) {
        self.get_hungry();
        self.hunger = (self.hunger - 30).max(0);
        self.status();
    }

    fn take_damage(&mut self, damage: i32) {
        println!("Took {damage} damage.");
        self.get_hungry();

        self.hp -= damage;
        if self.hp <= 0 {
            println!("DEAD!!! - Killed.");
            return;
        }
        self.status();
        }
    
    fn heal(&mut self, hp: i32) {
        println!("Healed {hp} HP points.");
        self.get_hungry();
        self.hp = (self.hp + hp).min(100);
        self.status();
        }

    fn status(&self) {
        println!("{} --> HP: {}, Hunger: {}", self.name, self.hp, self.hunger);
        }
    }




fn main() {
    let mut joe = Player::new("Joe");

    joe.take_damage(40);
    joe.heal(30);
    joe.eat();
    joe.take_damage(10);
    joe.take_damage(20);
    joe.take_damage(50);
    joe.heal(60);
    joe.eat();
}