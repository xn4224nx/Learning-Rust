use rand::seq::SliceRandom;
use rand::{self, Rng};

#[derive(Debug)]
struct Dwarf {}

#[derive(Debug)]
struct Elf {}

#[derive(Debug)]
struct Human {}

#[derive(Debug)]
enum Thing {
    Sword,
    Trinket,
}

trait Enchanter: std::fmt::Debug {
    fn competency(&self) -> f64;

    fn enchant(&self, item: &mut Thing) {
        let prob_of_success = self.competency();
        let spell_worked = rand::thread_rng().gen_bool(prob_of_success);

        print!("{:?} mutters incoherently. ", self);

        if spell_worked {
            println!("The {:?} glows brightly.", item)
        } else {
            println!("The {:?} fizzes, then turns into a worthless trinket", item);
            *item = Thing::Trinket {};
        }
    }
}

impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        return 0.5;
    }
}

impl Enchanter for Elf {
    fn competency(&self) -> f64 {
        return 0.95;
    }
}

impl Enchanter for Human {
    fn competency(&self) -> f64 {
        return 0.8;
    }
}

fn main() {
    let mut object = Thing::Sword;

    let d = Dwarf {};
    let e = Elf {};
    let h = Human {};

    let party: Vec<&dyn Enchanter> = vec![&d, &h, &e];

    let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();

    spellcaster.enchant(&mut object);
}
