mod ex_of_shadowing;
fn main() {
    //Imp note:- "mut" ae khali aena j type ni val. reassign karva de,jo biji kai to err. ape.
    /*O/P:-Err. aape bec. "immutability".*/
    // let a=25;
    // a=35 or true or "hello";
    // println!("Value of x is:- {a}");

    { /*O/P:-35 */ }
    // let mut a = 25;
    // a = 35;
    // println!("Value of x is:- {a}");

    { /*O/P:-Err. bec. float type */ }
    // let mut a = 25;
    // a = 35.15;
    // println!("Value of x is:- {a}");

    /*O/P:- Err because bool type ni val. */
    // let mut a=25;
    // a=true; //bool
    // println!("Value of x is:- {a}");

    /*O/P:- Err because bool type ni val. */
    // let a=25;
    // a=true; //bool
    // println!("Value of x is:- {a}");

    { /* Shadowing:Same name with same type or another type's val.But vari. ne diff. memory address hase */
    }
    let z = 12;
    let z = 25;
    println!("Value of z is:- {z}");

    //Same name with diff. type's val.
    let a = 24;
    let a = "jay";

    println!("Value of a is:- {a}");

    println!("** Below o/p is from other file **");
    ex_of_shadowing::example_of_shadowing();
}
