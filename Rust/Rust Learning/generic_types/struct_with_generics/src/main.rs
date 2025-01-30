use std::fmt::Debug;
/*Imp note:- #[derive(Debug]) isn't usefull for debug to "Generic type",instead of it we've to use "Debug" trait.*/

/*Points name struct is working for int & float both type val,but both value should be same type. */
struct Points<T> {
    x: T,
    y: T,
}

/*Student struct allow multiple types val. */
struct Student<T, U, V> {
    name: T,
    roll_num: U,
    bank_amt: V,
}

struct Employee<T, U> {
    id: T,
    name: U,
}

//Implementation for Associated Method
impl<T: Debug, U: Debug> Employee<T, U> {
    fn emp_details(id: T, name: U) {
        println!("Employee Id is:{:?} and name is:{:?}", id, name);
    }
}

fn main() {
    //For single type
    let point1 = Points { x: 10, y: 15 };

    println!("Point1 x is on: {}", point1.x);
    println!("Point1 y is on: {}", point1.y);

    let point2 = Points { x: 2.5, y: 11.2 };
    println!("Point2 x is on: {}", point2.x);
    println!("Point2 y is on: {}", point2.y);

    //For multiple type
    let student1 = Student {
        name: "Jay",
        roll_num: 5054,
        bank_amt: 1820.33,
    };

    println!("Student 1 name is: {}", student1.name);
    println!("Student 1 roll number is: {}", student1.roll_num);
    println!("Student 1 bank amount is: {}", student1.bank_amt);

    //For Associated Method.
    Employee::emp_details(12, String::from("Umang"));
}
