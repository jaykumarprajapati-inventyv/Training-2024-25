use std::io;
use tokio::time::{sleep, Duration};

#[tokio::main]

pub async fn start_level() {
    println!("** Game Level 3 – The Mountains of Memory **");
    sleep(Duration::from_millis(2000)).await;

    println!(
        "Once upon a time, in a picturesque and cool city, a former king announced a grand 
        competition: a mountain-climbing challenge. The winner of this competition would 
        gain not just glory but the ultimate prize — the rule of the entire kingdom! They would 
        be crowned permanently as the ruler."
    );
    sleep(Duration::from_millis(3000)).await;

    println!(
        "To ensure fairness, the king's ministers handed out a rule book to all competitors, 
        with a strict condition that it must be returned after being read. To manage this, 
        they implemented a concept called \"takes and gives back ownership.\" This allowed 
        competitors to temporarily take ownership of the rule book, read it, and then return 
        it when done."
    );
    sleep(Duration::from_millis(3000)).await;

    let rule_book_for_competition = [
        "1. Do not cheat with anyone.",
        "2. Complete the climb within 5 hours.",
        "3. You are solely responsible for your safety.",
        "4. One backpack per person at a time.",
        "5. Explicitly return borrowed gear after use.",
    ];

    sleep(Duration::from_millis(4000)).await;

    let get_back_rule_book = takes_and_gives_back_ownership(rule_book_for_competition);
    println!(
        "Thank you for reading and returning the rule book. Here it is again for reference: {:#?}",
        get_back_rule_book
    );

    sleep(Duration::from_millis(4000)).await;

    println!(
        "Additionally, the king's ministers provided a kit of mountain climbing equipment 
        for sale to the competitors. Each item in the kit is mutable, meaning competitors can 
        update or add quantities to suit their needs. This was achieved using the \"mutable reference 
        borrowing\" concept."
    );

    sleep(Duration::from_millis(4000)).await;

    println!(
        "Welcome, competitor! I am the shopkeeper, here to provide you with climbing equipment. 
        Simply tell me the item name and the quantity you need, and I’ll update your kit."
    );

    let mut mountain_climbing_equipments_kit = [
        ("Shoes pair", 1),
        ("Rope in feet", 10),
        ("Stainless steel snap hook", 1),
        ("Oxygen cylinder in liters", 700),
        ("Mini camping shovel", 1),
    ];

    sleep(Duration::from_millis(4000)).await;

    mutate_climbing_equipments_kit(&mut mountain_climbing_equipments_kit);

    sleep(Duration::from_millis(4000)).await;

    println!(
        "As promised, the winner of this competition will gain permanent control over the kingdom! 
        To bestow such an important prize, we use the ownership concept."
    );

    sleep(Duration::from_millis(3000)).await;

    println!(
        "The competition was fierce, and the mountain stood tall, testing the courage, strength, 
        and determination of every competitor. As the sun began its descent, a roar of applause echoed 
        through the valleys! One extraordinary climber emerged victorious, conquering the peak 
        within the challenging time limit of 5 hours!"
    );

    sleep(Duration::from_millis(3000)).await;

    println!(
        "With this incredible feat, the competitor not only proved their unmatched skill and resilience 
        but also earned the highest honor of the kingdom. The king, along with his court, stood in awe, 
        ready to crown the new ruler of the realm."
    );
    let reward = String::from("Crown");
    give_crown_ownership(reward);
}

fn mutate_climbing_equipments_kit(climbing_equipments_pack: &mut [(&str, i32)]) {
    println!(
        "Here are the current items in your equipment kit: {:#?}",
        climbing_equipments_pack
    );

    println!("Would you like to purchase any additional items? Please type \"yes\" or \"no\".");
    let mut choice: String = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Please enter valid input!");
    let choice = choice.trim().to_lowercase();

    match choice.as_str() {
        "yes" => {
            println!("Please provide the name of the item you'd like to purchase.");
            let mut item_name: String = String::new();
            std::io::stdin()
                .read_line(&mut item_name)
                .expect("Please input a valid item name!");
            let item_name = item_name.trim();

            for (item, qty) in climbing_equipments_pack.iter_mut() {
                match *item == item_name {
                    true => {
                        println!("How many quantities would you like to purchase?");
                        let mut quantity: String = String::new();
                        std::io::stdin()
                            .read_line(&mut quantity)
                            .expect("Please input a valid quantity!");
                        let quantity: i32 = quantity
                            .trim()
                            .parse()
                            .expect("Please enter a valid numeric value.");
                        *qty = quantity;
                        println!(
                            "Your equipment kit has been updated: {:#?}",
                            climbing_equipments_pack
                        );
                        return;
                    }
                    false => {
                        continue;
                    }
                }
            }
            println!("Sorry, the item \"{item_name}\" is not available.");
        }
        "no" => {
            println!(
                "Thank you for browsing! Feel free to come back anytime if you need more equipment."
            );
        }
        _ => {
            println!("Invalid choice. Please enter \"yes\" or \"no\".");
        }
    }
}

fn takes_and_gives_back_ownership(rule_book: [&str; 5]) -> [&str; 5] {
    println!(
        "Please read the rule book carefully before beginning your climb. Remember to return 
        it to the king's ministers once you're done!"
    );
    rule_book
}

fn give_crown_ownership(reward: String) {
    println!(
        "Congratulations! You are now the permanent ruler of the kingdom, and you have received 
        the royal {reward} as a symbol of your authority!"
    );
}
