use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]

enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn multiple_ownership_with_mutability() {
    let val = Rc::new(RefCell::new(5));

    let a=Rc::new(Cons(Rc::clone(&val), Rc::new(Nil)));

    let b=Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c=Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *val.borrow_mut()+=10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");



}
