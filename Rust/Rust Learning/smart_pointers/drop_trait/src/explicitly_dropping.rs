use std::mem::drop;

struct CustomSmartPointer {
    value: String,
}

pub fn explicit_dropping() {
    let s1 = CustomSmartPointer {
        value: String::from("Explicitly dropping"),
    };

    println!("CustomSmartPointer created.");
    drop(s1);
    println!("CustomSmartPointer dropped before end of function.");
}
