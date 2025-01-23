pub fn for_loops() {
    let arr = [20, 30, 40, 50, 60];

    for elem in arr {
        println!("Elements through for loop:{elem}")
    }

    println!("rev() in for loop");

    for num in (1..4).rev() {
        println!("Using rev():{num}");
    }
}
