use std::ops::Deref;
#[derive(Debug)]

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(val: T) -> MyBox<T> {
        //This function returns a MyBox<T> instance.
        MyBox(val) //Creates a new instance of MyBox containing val.
    }
}

/* Role of Deref Trait
 > Allows MyBox<T> to behave like a reference.
 > When we use *z, it calls "deref()" automatically to get the inner value.
*/

/* type Target = T;
> "type" is declaring an associated type inside the trait implementation.
> "Target" is just the name we give to that associated type.
> Whenever we use deref(), it will return a T type.
 */

impl<T> Deref for MyBox<T> {
    type Target = T; /*"type" is declaring an associated type inside the trait implementation.
                     "Target" is just the name we give to that associated type. */

    fn deref(&self) -> &T {
        &self.0 // Returns a reference of the param's value.
    }
}
fn main() {
    /* Normal way */
    let a = 5;
    let b = &a;

    assert_eq!(a, 5);
    assert_eq!(a, *b);
    // assert_eq!(a, b); It'll throw an err. bec. "b" will work as "address of a's val.".

    /* By using Built-in Smart Pointer */
    let x = 10;
    let y = Box::new(x);

    assert_eq!(x, 10);
    assert_eq!(x, *y);
    // assert_eq!(x,y); It'll throw an err. bec. "y" will work as "address of x's val.".

    /* By using Custom Smart Pointer */
    let z = MyBox::new(10);
    println!("Value inside MyBox is:-{}", *z);

    assert_eq!(10, *z); // Deref allows *z to access `10`
                        // assert_eq!(10, z); It'll throw an err. bec. "z" will work as "address of z's val.".
}
