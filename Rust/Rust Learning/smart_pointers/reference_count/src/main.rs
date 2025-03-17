use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
fn main() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))))); //Reference count 1

    let b = Rc::new(Cons(4, Rc::clone(&a))); //Reference count 2

    let c = Rc::new(Cons(5, Rc::clone(&a))); //Reference count 3

    {
        let d = Rc::new(Cons(6, Rc::clone(&a)));
        println!("Total Reference counting of a is:-{}", Rc::strong_count(&a));//Reference count 4
    } 

    println!("b={:?}", b);
    println!("c={:?}", c);
    println!("Now Reference counting of a is {} because d is out of scope.", Rc::strong_count(&a));
}
