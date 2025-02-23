/*
Absolute path:- For this "crate::"" is used to start from the crate root and access modules, structs, functions, or other items, ensuring you always get the correct path, no matter where you are in the module hierarchy.
  >E.x.:- crate::front_of_house::hosting::add_to_waitlist();

Relative path:- It starts from the module name and uses self,super, or an identifier in the current module.
  > E.x.:- front_of_house::hosting::add_to_waitlist();
*/

//use std::{cmp::Ordering, io}; :-We can use nested paths to bring the same items into scope in one line.

//(1) To create module we used "mod" keyword, inside code of module is "private" by default. We've to use "pub" so other can access it.
mod inner_module {
    pub fn greet() {
        println!("Hello! Good morning");
    }
}

//(2)Here, we use outer module's code to a separate file for better organization.
mod outer_module;

//(3)The "as" keyword allows you to give a different name (alias) to an imported module, function, or struct. "as" keyword is used only with the "use" statement to rename imports.
mod use_and_as_keyword;
use use_and_as_keyword::msg as alias;

//(4)
mod struct_enum;

//(5)
mod super_keyword;

//(6)
mod pub_use;

fn main() {
    //(1)
    inner_module::greet();
    //(2)
    outer_module::my_module();
    //(3)
    alias();
    //(4) Struct and enum
    let item1 = struct_enum::restaurant::prepare_dish();
    println!(
        "Serving {} as {:?} (struct and enum)",
        item1.item_name, item1.serve_way
    );

    //Absoulate path
    crate::super_keyword::parent::call_child();

    //pub use
    pub_use::student_rollno();
}
