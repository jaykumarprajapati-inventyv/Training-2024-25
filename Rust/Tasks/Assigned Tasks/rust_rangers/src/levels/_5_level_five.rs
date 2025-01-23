use std::io;
use tokio::time::{sleep, Duration};

struct Adventure {
    name: String,
    power: i32,
    weapon: String,
}
enum MagicItem {
    Sword,
    Shield,
    Potion,
}

impl Adventure {
    fn strengths(&self) {
        println!(
            "Dear {}, you have {} power and a solid {} in your magic weapon box. It will help you become the ultimate winner against any monster!",
            self.name, self.power, self.weapon
        );
    }
}

impl MagicItem {
    fn weapon_name(name: String) -> Self {
        match name.as_str() {
            "Sword" => MagicItem::Sword,
            "Shield" => MagicItem::Shield,
            "Potion" => MagicItem::Potion,
            _ => panic!("Invalid weapon name! Please try again with a valid command."), //It gives panic error when..
        }
    }
}

#[tokio::main]
pub async fn start_level() {
    println!("\n \n Welcome to the Adventure of Heroes! Embark on an epic journey where you’ll face three thrilling challenges to prove your courage, intelligence, and strength. Armed with magical tools like the Sword, Shield, and Potion, you'll battle a fierce Monster, outsmart a blazing Fire Trap, and heal yourself from the venom of a Poisonous Snake. If you win every challenges then you'll get Golden Crown!");

    println!("\nWelcome! Please enter your name so we can begin this epic journey together.");
    let mut name: String = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Please enter a valid name.");

    let name = name.trim();

    let mut user = Adventure {
        name: name.to_string(),
        power: 100,
        weapon: String::from("Sword"),
    };

    sleep(Duration::from_millis(1000)).await;

    println!("\n");
    user.strengths();

    sleep(Duration::from_millis(3000)).await;

    challenge_one_the_monster(&user.name, &mut user.power).await;

    sleep(Duration::from_millis(4000)).await;
    challenge_two_the_monster(&user.name, &mut user.power).await;

    sleep(Duration::from_millis(4000)).await;
    challenge_three_the_monster(&user.name, &mut user.power).await;

    sleep(Duration::from_millis(3000)).await;
    println!("\nCongratulations, {}! You’ve proven yourself to be a true hero! With courage, wisdom, and the perfect use of your magical tools, you’ve overcome every challenge in your path. From defeating the fierce monster to outsmarting the fire trap and healing from the snake’s venom, you’ve shown that anything is possible when bravery meets skill. The Golden Crown is yours! Wear it proudly.", user.name);
}

fn get_weapon(weapon: MagicItem, power: &mut i32) {
    match weapon {
        MagicItem::Sword => {
            println!("You got the Sword! Now, you’ll triumph against the monster!"); //Here,semicolon is compulsory to terminate.
            *power -= 10;
        }
        MagicItem::Shield => {
            println!("You got the Shield! Now, you’ll survive the fire trap!");
            *power -= 10;
        }
        MagicItem::Potion => {
            println!("You got the Potion! You feel rejuvenated and ready for battle!");
            *power += 10;
        }
    }
}

async fn challenge_one_the_monster(player_name: &String, player_power: &mut i32) {
    println!("\n\n*****                  Challenge 1: The Monster!                  *****");
    sleep(Duration::from_millis(2000)).await;

    println!("\nSuddenly, a wild monster appears! It’s big, scary, and ready to fight. You need to use your Sword to defeat the monster.");
    sleep(Duration::from_millis(3000)).await;

    println!("\nEnter the command \"Sword\" to retrieve your magical Sword and defeat the monster. Do not enter any other command!");
    let mut weapon: String = String::new();

    io::stdin().read_line(&mut weapon).expect(
        "You gave the wrong command to your magic weapon box! The game will now terminate.",
    );

    weapon = weapon.trim().to_string();
    let weapon = format!(
        "{}{}",
        weapon.chars().next().unwrap().to_uppercase(),
        &weapon[1..]
    );
    let item = MagicItem::weapon_name(weapon);
    sleep(Duration::from_millis(2000)).await;

    println!("\n");
    get_weapon(item, player_power);
    sleep(Duration::from_millis(4000)).await;

    println!("\n\n** Congratulations, {}! You defeated the giant monster and emerged victorious! You have {} power left. Get ready for the next challenge. Keep up the great work! **", player_name, player_power);
}

async fn challenge_two_the_monster(player_name: &String, player_power: &mut i32) {
    println!("\n\n*****                  Challenge 2: The Fire Trap!                  *****");

    sleep(Duration::from_millis(2000)).await;

    println!("\nNext, you find a dangerous fire trap blocking your path. You need a Shield to protect yourself.");
    sleep(Duration::from_millis(3000)).await;

    println!("\nEnter the command \"Shield\" to retrieve your magical Shield and protect yourself. Do not enter any other command!");
    let mut weapon: String = String::new();

    io::stdin().read_line(&mut weapon).expect(
        "You gave the wrong command to your magic weapon box! The game will now terminate.",
    );

    weapon = weapon.trim().to_string();
    let weapon = format!(
        "{}{}",
        weapon.chars().next().unwrap().to_uppercase(),
        &weapon[1..]
    );
    let item = MagicItem::weapon_name(weapon);
    sleep(Duration::from_millis(2000)).await;

    println!("\n");

    get_weapon(item, player_power);
    sleep(Duration::from_millis(4000)).await;
    println!("\n");

    println!("** Congratulations, {}! You’ve successfully navigated the fire trap and emerged victorious in the second challenge! You now have {} power remaining. Prepare yourself for the final challenge. You’re almost there! **", player_name, player_power);
}

async fn challenge_three_the_monster(player_name: &String, player_power: &mut i32) {
    println!("\n\n*****                  Challenge 3: The Poisonous Snake!                  *****");

    sleep(Duration::from_millis(2000)).await;

    println!("\nFinally, you come across a poisonous snake. You’ve taken some damage from the previous battles, so you need a Potion to heal.");
    sleep(Duration::from_millis(3000)).await;

    println!("\nEnter the command \"Potion\" to retrieve your magical Potion and restore your health. Do not enter any other command!");
    let mut weapon: String = String::new();

    io::stdin().read_line(&mut weapon).expect(
        "You gave the wrong command to your magic weapon box! The game will now terminate.",
    );

    weapon = weapon.trim().to_string();
    let weapon = format!(
        "{}{}",
        weapon.chars().next().unwrap().to_uppercase(),
        &weapon[1..]
    );
    let item = MagicItem::weapon_name(weapon);
    sleep(Duration::from_millis(3000)).await;
    println!("\n");

    get_weapon(item, player_power);
    sleep(Duration::from_millis(4000)).await;
    println!("\n");

    println!("\n** Amazing job, {}! You’ve conquered the final challenge and earned +10 power points as a reward! Your final power is {}. Congratulations, hero! **", player_name, player_power);
}
