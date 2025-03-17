use std::io;

pub fn prime() {
    let mut num: String = String::new();
    let mut count = 0;

    println!("Enter a number:");
    io::stdin().read_line(&mut num).expect("Invalid input");

    let num: i32 = num
        .trim()
        .parse()
        .expect("Can't possible to convert into num type!!");

    for i in 1..=num {
        if num % i == 0 {
            count += 1;
        }
    }

    if count == 2 {
        println!("Yes {} is prime!!", num);
    } else {
        println!("No {} isn't prime!!", num);
    }
}
