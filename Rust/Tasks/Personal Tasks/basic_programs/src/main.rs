use std::io;

mod count_digit_in_num;
mod factorial;
mod fibonacci_series;
mod palindrome;
mod perfect_num_or_not;
mod prime_num;
mod swap_to_num;

fn main() {
    // factorial::factorial();
    // prime_num::prime();
    // fibonacci_series::fibonacci_series();
    // swap_to_num::swapping();
    perfect_num_or_not::perfect_no();
    count_digit_in_num::count_digit_in_input_num();

    /* Check Palindrom or not */
    let mut s1 = String::new();

    io::stdin().read_line(&mut s1).expect("Invalid input!!");
    let mut s1: String = s1.trim().parse().expect("Not possible to convert!!");

    let is_palindrome = palindrome::check_palindrome(&mut s1);
    println!("Palindrome or not ?={is_palindrome}");
}
