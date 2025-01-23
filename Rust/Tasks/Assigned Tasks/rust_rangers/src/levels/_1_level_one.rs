use std::{cmp::Ordering, io};
use tokio::time::{sleep, Duration};

#[tokio::main]

pub async fn start_level() {
    println!("** Game Level 1!! **");
    sleep(Duration::from_millis(2000)).await;
    const _FOREST_AREA_IN_ACRES: i32 = 2000;
    let max_deer: i32 = 500;
    
    // Mutating the original deer count; passing its reference to the function.
    let mut deer = 400;

    println!("** In the forest, there are a total of \"{deer} Deer\" and the forest area is 2000 acres. The deer population should be up to 500. If it exceeds 500, it will give warning to you. **");
    grow_population_deer(&mut deer, max_deer).await;
    reduce_population_deer(&mut deer).await;
}

// Taking a reference to `deer` and modifying it using dereference (*).
async fn grow_population_deer(deer: &mut i32, max_deer: i32) {
    println!("How many deer will you take from the zoo?");
    let mut from_zoo: String = String::new();
    io::stdin()
        .read_line(&mut from_zoo)
        .expect("Please enter a valid input");

    let from_zoo: i32 = from_zoo
        .trim()
        .parse()
        .expect("Failed to parse! Please enter a valid number.");

    match (*deer + from_zoo).cmp(&max_deer) {
        Ordering::Less => {
            *deer += from_zoo;
            sleep(Duration::from_millis(2000)).await;
            println!("After adding, the total deer count is: {deer}");
        }
        Ordering::Greater => {
            println!("Warning!! You can only grow the deer population up to 500. No deer added.");
        }
        Ordering::Equal => {
            println!("The total deer count is now 500. You cannot add more deer.");
        }
    }
}

async fn reduce_population_deer(deer: &mut i32) {
    println!("Enter 1 for hunting, or 2 for taking deer to the zoo.");
    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Please enter a valid input");

    let choice: i32 = choice
        .trim()
        .parse()
        .expect("Failed to parse! Please enter a valid number.");

    match choice {
        1 => {
            println!("Are you sure you want to hunt?");
            let mut option = String::new();
            io::stdin()
                .read_line(&mut option)
                .expect("Please enter a valid input!");

            let option = option.trim().to_lowercase();

            //"as_str()" converts a String to &str.
            match option.as_str() {
                "yes" => {
                    println!("How many deer do you want to hunt?");

                    let mut hunting: String = String::new();

                    io::stdin()
                        .read_line(&mut hunting)
                        .expect("Please enter a valid input!");

                    let hunting: i32 = hunting
                        .trim()
                        .parse()
                        .expect("Please enter a valid number!");

                    *deer -= hunting;
                    sleep(Duration::from_millis(2000)).await;

                    println!("The total deer count is: {deer}");
                }
                "no" => {
                    println!("Thank you for choosing not to hunt!");
                }
                _ => {
                    println!("Invalid choice! Please enter 'yes' or 'no'.");
                }
            }
        }
        2 => {
            println!("How many deer do you want to take to the zoo?");

            let mut take: String = String::new();

            io::stdin()
                .read_line(&mut take)
                .expect("Please enter a valid input!");

            let take: i32 = take.trim().parse().expect("Please enter a valid number!");

            *deer -= take;
            sleep(Duration::from_millis(2000)).await;

            println!("Now the total deer count is: {deer}");
        }
        _ => {
            println!("Invalid choice! Please enter 1 or 2.");
        }
    }
}
