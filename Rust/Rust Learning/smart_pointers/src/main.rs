/*Here, 'a' is Box which stored in stack and "value 10" which stored in Heap,Box is one type pointer,which point to Heap's val.  */

/*Here, enum'll throw an error bec. its Cons's List has infinite size, so compiler can't know its size at compile time and compiler want fixed size at compilation time. It also recursive. It storing itself within itself. */

#[derive(Debug)]
// enum List {
//     Cons(i32, List),
//     Nil,
// }

/*Solution of it is "Box<List>", it is pointer which just point to one by one. */
enum List {
    Cons(i32, Box<List>),
    Nil,
}
fn main() {
    //1
    let a = Box::new(10);
    println!("Value of Box which stored in Stack:-{a}");
    println!("Value which stored in Heap:-{}", *a);

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
