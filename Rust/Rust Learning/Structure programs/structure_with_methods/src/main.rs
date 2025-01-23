struct Rectangle {
    length: u32,
    width: u32,
}

//Ahi me 2 implement, method ma multiple paramter and mutability of self batavyu chhe.

//Mutability of self je bydefault immutable hoi chhe.
impl Rectangle {
    fn area_of_rectangle(&mut self) -> u32 {
        self.length += 10;
        self.length * self.width
    }
}

impl Rectangle {
    fn checker_larger_or_not(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
//** Multiple parameters in method **

fn main() {
    let mut area1 = Rectangle {
        length: 210,
        width: 100,
    };

    let area2 = Rectangle {
        length: 150,
        width: 90,
    };
    println!("Area of rectangle is:{:?}", area1.area_of_rectangle());

    println!(
        "Is area2's fields's value are large?:{:?}",
        area1.checker_larger_or_not(&area2)
    );
}
