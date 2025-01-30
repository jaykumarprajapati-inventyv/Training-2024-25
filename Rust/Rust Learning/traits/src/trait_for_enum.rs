trait Animalsound {
    fn sound(&self) -> String;
}

#[derive(Debug)]
enum Animal {
    Lion,
    Dog,
    Cat,
}

impl Animalsound for Animal {
    fn sound(&self) -> String {
        match self {
            Animal::Lion => String::from("Roars"),
            Animal::Dog => String::from("Barks"),
            Animal::Cat => String::from("Meowing"),
        }
    }
}

pub fn demo() {
    let a = Animal::Dog;
    println!("Sound of {a:?} is:{}", a.sound());
}
