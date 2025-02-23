use std::{
    fs::File,
    io::{self, Read},
};
pub fn propagating_the_error() -> Result<String, io::Error> {
    let file_res = File::open(
        "D:/Inventyv/Tasks/Training-2024-25/Rust/Rust Learning/error_handling/src/hello.txt",
    );

    let mut ans: File = match file_res {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut content = String::new();

    match ans.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}
