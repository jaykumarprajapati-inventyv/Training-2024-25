use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("** Guessing Game!! **");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("secret_num:- {secret_num}");

    loop {
        println!("Enter your Guess number:");

        let mut guess_num: String = String::new();

        io::stdin()
            .read_line(&mut guess_num)
            .expect("Failed to take input");

        println!("You input:- {guess_num}");

        /*Ahi niche upar na guess_num ni shadow bani,hve thi aa seco. nu j usefull rehse,
        ae immutable banyu .*/
        let guess_num: u32 = match guess_num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess_num.cmp(&secret_num) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too large!!"),
            Ordering::Equal => {
                println!("You won!!");
                break;
            }
        }
    }
}
