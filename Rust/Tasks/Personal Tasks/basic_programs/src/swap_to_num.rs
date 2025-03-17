pub fn swapping() {
    let mut a = 10;
    let mut b = 15;
    let c;

    println!("Before a={a}");
    println!("Before b={b}");
    c = a;
    a = b;
    b = c;

    println!("After a={a}");
    println!("After b={b}");
}
