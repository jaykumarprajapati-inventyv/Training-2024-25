use std::io;

pub fn fibonacci_series() {
    //Created all vari. in one line using "tuple destructure"
    let (mut a, mut b, mut c) = (0, 1, 0);

    println!("Enter a number");
    let mut no = String::new();

    io::stdin().read_line(&mut no).expect("Invalid input");

    let no: i32 = no.trim().parse().expect("Not possible to convert!!");

    print!("\n");

    for _ in 1..=no {
        println!("{}", a);
        c = a + b;
        a = b;
        b = c;
    }
}
