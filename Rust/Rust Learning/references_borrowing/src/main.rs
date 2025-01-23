fn main() {
    let s1 = String::from("Hello");
    let mut s2 = String::from("Jay");

    //Immutable References in paramter
    immutable_borrow_func(&s1);

    //Mutable References in parameter
    mutable_borrow_func(&mut s2);

    //Two vari. using mutable refrence of same variable.
    //Aa niche nu erro. aape,bec. bane sathe na chale atle aene alag karva local scope banavi ne juda padva pade.
    // let val1 = &mut s2;
    // let val2 = &mut s2;

    {
        let val1 = &mut s2;
        println!("{}", val1);
    }
    let val2 = &mut s2;

    println!("{}", val2);
}
fn immutable_borrow_func(s: &String) {
    println!("Simple immutable reference value:{s}");
}

fn mutable_borrow_func(s: &mut String) {
    s.push_str(" Prajapati");
    println!("Simple mutable reference value:{s}");
}
