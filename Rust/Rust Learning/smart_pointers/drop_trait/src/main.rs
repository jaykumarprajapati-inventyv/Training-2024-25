//Generally, In each and every program Rust uses "Drop trait" automatically. So, even without Drop implementation, Rust automatically cleans up variables when they go out of scope.
mod explicitly_dropping;
struct CustomSmartPointer {
    value: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        //It always take "mutable self" as parameter.
        println!("Dropping {} Data of Custom smart pointer.", self.value);
    }
}

fn main() {
    println!("** Below o/p is from \" Explicitly dropping \" **");
    explicitly_dropping::explicit_dropping();

    println!("\n ** Below o/p is from \" Implicitly dropping \" **");
    println!("Created instance of struct which are dropping in \"Reverse order\".");

    let s1 = CustomSmartPointer {
        value: String::from("First Instance"),
    };

    let s2 = CustomSmartPointer {
        value: String::from("Second Instance"),
    };
}
