use std::{
    fs::File,
    io::{self, Read},
};

//Here, If an error occurs, "?" stops execution and returns the error immediately.
pub fn propagating_the_error_by_shortcut_way() -> Result<String, io::Error> {
    let mut inner_content = String::new();
    
    File::open(
        "D:/Inventyv/Tasks/Training-2024-25/Rust/Rust Learning/error_handling/src/hello.txt",
    )?
    .read_to_string(&mut inner_content)?;

    Ok(inner_content)
}
