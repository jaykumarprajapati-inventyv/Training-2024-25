use std::io;
pub fn factorial() {
    let mut fact = 1;

    println!("Enter a num:");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Please enter valid number");

    let num: i32 = num.trim().parse().expect("Can't possible to convert");

    println!("Entered num is:{num}");
    for i in (1..=num).rev() {
        fact = fact * i;
    }
    println!("Factorial of {num} is:-{fact}");
}
