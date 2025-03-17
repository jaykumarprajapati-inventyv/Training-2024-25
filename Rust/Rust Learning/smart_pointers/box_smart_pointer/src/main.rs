/*Here, 'a' is Box which stored in stack and "value 10" which stored in Heap,Box is one type pointer,which point to Heap's val.  */

/*Here, enum'll throw an error bec. its Cons's List has infinite size, so compiler can't know its size at compile time and compiler want fixed size at compilation time. It also recursive. It storing itself within itself. */

/*
  Box<T> use cases as below:-
    (a) Heap Allocation is Needed – For large data that should not be moved frequently.
    (b) Recursive Types – To give a fixed size to recursive types like linked lists, so compiler can aware at compile time.
    (c) Trait Objects (dyn Trait) – When storing different types that implement the same trait.
*/
/*In enum "List" is enum name and also it defines the type of values it can store. */
#[derive(Debug)]
// enum List {
//     Cons(i32, List),
//     Nil,
// }

/*Solution of it is "Box<List>", it is pointer which just point to one by one. */
/*
How Box<T> Fixes error?
 > Box<T> stores the data on the heap and only a fixed-size pointer remains in the enum, pointer size will be 8 bytes bec. we've 64-bit,so here compiler get it thats why it'll not throw error.
 > Since the pointer has a known size, Rust stops the infinite recursion error.
*/
enum List {
    Cons(i32, Box<List>),
    Nil,
}
fn main() {
    //1
    let a = Box::new(10);
    println!("Value of Box which stored in Stack:-{a}");
    println!("Value which stored in Heap:-{}", *a);

    let b = a;
    // println!("Using a after assign:-{}", a); //Aa err. ape bec. ahi "a" hve int nathi pn Box chhe jeni val. heap ma chhe atle,ae "copy trait" ne follow nathi kartu.

    //2
    // let list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));
    // println!("List is:{:?}", list);

    // 3
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("List is:{:?}", list);
}
