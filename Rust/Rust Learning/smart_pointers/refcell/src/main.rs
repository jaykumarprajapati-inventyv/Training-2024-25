use std::cell::RefCell;
mod multiple_ownership_and_mutablity;
fn main() {
    //    let a=5;
    //    a=15;
    //    println!("a={a}"); Throw an err. bec. we can't mutate to immutable vari.

/*So thats why, if we want to "mutate" to immutable vari. at runtime, we've to use "RefCell".*/

    let a = RefCell::new(5);
    // let b=&a;
    *a.borrow_mut() += 5;
    println!("a={:?}", a.borrow());

    println!("** Multiple ownership and mutability. **");
    multiple_ownership_and_mutablity::multiple_ownership_with_mutability();

/*If we're trying to create two mutable reference in same scope, then panic will be occur! as below code. */
    let x=RefCell::new(20);
    let mut y=x.borrow_mut();
    let mut z=x.borrow_mut();

    *y+=5;
    *z+=7;
    println!("y={y}");
    println!("z={z}");

}
