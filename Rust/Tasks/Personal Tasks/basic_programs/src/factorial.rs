use std::io;
pub fn factorial() {
    let mut fact = 1;

    println!("Enter a number");
    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("Invalid input");

    let num: i32 = num
        .trim()
        .parse()
        .expect("Not possible to convert into number type.");

    for i in (1..=num).rev() {
        fact = fact * i;
    }

    println!("Factorial of {} is {}", num, fact);
}
