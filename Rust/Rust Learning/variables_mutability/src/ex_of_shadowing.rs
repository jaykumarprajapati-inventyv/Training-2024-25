pub fn example_of_shadowing() {
    let x = 5;

    let x = x + 2; //7

    {
        let x = x * 2;
        println!("Local x is :- {x}"); //14
    }
    println!("Val. of x is:-{x}"); //7
}
