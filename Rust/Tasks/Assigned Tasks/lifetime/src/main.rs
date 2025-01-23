struct Car {
    name: String,
    color: String,
    // parts:&Parts,
}
struct Parts {
    no_of_seat: i32,
    engine: String,
}

fn main() {
    let c1 = Car {
        name: String::from("Fortuner"),
        color: String::from("White"),
    };
    println!("Name of car is:{}", c1.name);
    println!("Color of car is:{}", c1.color);

    let p1=Parts{
        no_of_seat:5,
        engine:String::from("Original")
    };

    println!("No of seats are:{}",p1.no_of_seat);
    println!("Engine is:{}",p1.engine);
}
