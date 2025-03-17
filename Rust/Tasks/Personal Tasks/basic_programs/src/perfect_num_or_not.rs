use std::io;
pub fn perfect_no() {
    println!("Enter a number:-");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Enter valid input!!");

    let num: i32 = num.trim().parse().expect("Can't possible to convert");

    let mut sum = 0;
    for i in 1..num {
        if num % i == 0 {
            sum += i;
        }
    }

    if sum == num {
        println!("Yes,{num} is Perfect");
    } else {
        println!("No,{num} isn't Perfect");
    }
}
