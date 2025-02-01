trait Trial {
    fn reveal_abilities(&self) -> String;
}

trait Fighters {
    fn share_details(&self) -> String;
}

use tokio::time::{sleep, Duration};
#[derive(Debug)]

enum ElementalPower {
    FireBreathing,
    LightningStrike,
    Magicalstick,
    Swords,
}

struct Monster {
    monster_name: String,
    monster_size_in_kg: i32,
    monster_power: ElementalPower,
}

struct Beast {
    creature_name: String,
    coat_color: String,
}

struct Guardian {
    creature_name: String,
    creature_size: i32,
    elemental_power: ElementalPower,
}

impl Trial for Beast {
    fn reveal_abilities(&self) -> String {
        format!(
            "Creature's name is: {} and it has a {} coat.",
            self.creature_name, self.coat_color
        )
    }
}

impl Trial for Guardian {
    fn reveal_abilities(&self) -> String {
        format!(
            "{} is {} meters tall and harnesses {:?} power.",
            self.creature_name, self.creature_size, self.elemental_power
        )
    }
}

impl Fighters for Guardian {
    fn share_details(&self) -> String {
        format!(
            "{} is {} meters tall and harnesses {:?} power.",
            self.creature_name, self.creature_size, self.elemental_power
        )
    }
}

impl Fighters for Monster {
    fn share_details(&self) -> String {
        format!(
            "{} is {} meters tall and harnesses {:?} power.",
            self.monster_name, self.monster_size_in_kg, self.monster_power
        )
    }
}

#[tokio::main]
pub async fn start_level() {
    println!(
        "***                      Level 6: The Trials of Mystic Creatures                      ***"
    );
    sleep(Duration::from_millis(1000)).await;

    println!("\nWelcome to *The Caverns of Traits*! In this game, you are a hero who will meet magical creatures, powerful guardians, and mighty monsters. As you journey through the world, you will discover their special powers and see them transform in exciting ways!");
    sleep(Duration::from_millis(3000)).await;

    let dragon_beast = Beast {
        creature_name: String::from("Flame Dragon"),
        coat_color: String::from("Red"),
    };
    let unicorn_beast = Beast {
        creature_name: String::from("Spark Unicorn"),
        coat_color: String::from("Pink"),
    };

    println!("\n** Scene 1: Meeting the Beasts **");
    sleep(Duration::from_millis(2000)).await;

    println!("To show the power of two creatures, I used the \"Trial trait\" through which creatures can reveal their special powers. The \"reveal_abilities function\" is part of this trait. The \"display_creature_abilities function\" makes two creatures show their abilities by calling this function.");
    sleep(Duration::from_millis(3000)).await;

    display_creature_abilities(&dragon_beast, &unicorn_beast);
    sleep(Duration::from_millis(3000)).await;

    println!("The Flame Dragon breathes fire with its power of Fire Breathing. The Spark Unicorn strikes with the power of Lightning!");

    let fire_dragon_guardian = Guardian {
        creature_name: dragon_beast.creature_name,
        creature_size: 35,
        elemental_power: ElementalPower::FireBreathing,
    };
    let lightning_unicorn_guardian = Guardian {
        creature_name: unicorn_beast.creature_name,
        creature_size: 15,
        elemental_power: ElementalPower::LightningStrike,
    };
    let wizard = Guardian {
        creature_name: String::from("Spell James"),
        creature_size: 90,
        elemental_power: ElementalPower::Magicalstick,
    };

    sleep(Duration::from_millis(2000)).await;

    println!("\n** Scene 2: The Guardians' Awakening **");
    sleep(Duration::from_millis(3000)).await;

    println!("We have a function called \"combine_guardian_powers\". This function is used to combine the powers of two guardians. It asks both guardians to reveal their abilities using the \"reveal_abilities function\"");
    sleep(Duration::from_millis(3000)).await;

    println!("Suddenly, two mighty guardians appear. The Fire Dragon Guardian stands at 35 meters tall, and the Lightning Unicorn Guardian stands tall at 15 meters. They show you the power they possess through the merging of their elemental abilities.");
    sleep(Duration::from_millis(2000)).await;

    combine_guardian_powers(&fire_dragon_guardian, &lightning_unicorn_guardian);
    sleep(Duration::from_millis(2000)).await;

    println!("The combined energy of fire and lightning shakes the ground beneath your feet. The guardian's power is now yours to command!");

    let monster_elements = Monster {
        monster_name: String::from("Knight Master"),
        monster_size_in_kg: 400,
        monster_power: ElementalPower::Swords,
    };

    sleep(Duration::from_millis(2000)).await;

    println!("\n** Scene 3: Facing the Knight Master **");
    sleep(Duration::from_millis(3000)).await;

    println!("We have a function called \"monster_knight\" that shows the details of a monster. The function asks the monster to share its details using the \"share_details function\".");
    sleep(Duration::from_millis(3000)).await;

    monster_knight(&monster_elements);
    sleep(Duration::from_millis(2000)).await;

    println!("The Knight Master’s sword strikes with deadly precision. But just as you prepare for battle, you realize you have powerful allies ready to help you in this challenge!");

    sleep(Duration::from_millis(3000)).await;

    println!("\n** Scene 4: Learning from the Wizard **");
    sleep(Duration::from_millis(2000)).await;

    println!("We have a function called \"multiple_trait\". This function works with things that have both the \"Trial trait and the Fighters trait\".");
    sleep(Duration::from_millis(2000)).await;

    println!("As you prepare for the final trial, a wise and powerful wizard named Spell James appears before you. Standing at 90 meters tall, he reveals his magical stick that grants him immense power.");
    sleep(Duration::from_millis(3000)).await;

    multiple_trait(&wizard);
    sleep(Duration::from_millis(2000)).await;

    println!("Spell James shares his knowledge, teaching you how to harness the combined powers of the creatures and guardians you’ve met. With his help, you are now prepared for the ultimate battle in the caverns!");

    sleep(Duration::from_millis(3000)).await;

    println!("\n** The Final Challenge **");
    sleep(Duration::from_millis(2000)).await;

    println!("Congratulations, Hero! You have learned how to harness the powers of all the creatures and guardians you’ve encountered. You've mastered the use of traits, functions, and combining powers. With the Fire Dragon’s flame, the Unicorn’s lightning, and the Wizard’s magic, you are now ready to face the most difficult trials in *The Caverns of Traits*.");

    sleep(Duration::from_millis(2000)).await;

    println!("\nYour journey doesn't end here! You've unlocked the ability to combine all these powers into one final weapon. This is the key to defeating the last monster of the cavern! You are now unstoppable!");
    sleep(Duration::from_millis(2000)).await;

    println!("\n** THE END? **");
    sleep(Duration::from_millis(1000)).await;

    println!("Or is it just the beginning of your next adventure? With the knowledge of combining traits and powers, you’ll face even stronger monsters, unlock new abilities, and go on new epic quests. Good luck, and may the traits be with you!");
    sleep(Duration::from_millis(3000)).await;

    println!("\n** You did it! You've completed this level, but a whole new world of adventure awaits you! **");
}

fn display_creature_abilities(creature1: &impl Trial, creature2: &impl Trial) {
    println!("{}", creature1.reveal_abilities());
    println!("{}", creature2.reveal_abilities());
}

fn combine_guardian_powers<T: Trial>(guardian1: &T, guardian2: &T) {
    println!("Combining the powers...");
    println!("Guardian 1: {}", guardian1.reveal_abilities());
    println!("Guardian 2: {}", guardian2.reveal_abilities());
}

fn monster_knight(monster: &Monster) {
    println!("Monster's details: {}", monster.share_details());
}

fn multiple_trait<T: Trial + Fighters>(item: &T) {
    println!("Trial and Fighters: {}", item.share_details())
}
