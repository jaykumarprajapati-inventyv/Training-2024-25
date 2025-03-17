use std::io;

pub fn count_digit_in_input_num() {
    println!("Enter a number:");

    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Invalid input");

    let mut num: i32 = num.trim().parse().expect("Can't convert to number");

    let mut count = 0;

    while num > 0 {
        num /= 10;
        count += 1;
    }

    println!("Total digits: {}", count);
}

fn main() {
    count_digit_in_input_num();
}
