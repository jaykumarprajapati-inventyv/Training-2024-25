use std::{fs::File, io::ErrorKind};

/*If file isn't exist, then we'll create the file so by this way we're handling to error. 
  > In error.kind() kind is method of struct, it returns to "ErrorKind".
  > Here, NotFound is a variant of the ErrorKind enum.
*/
pub fn matching_with_diff_err() {
    let file_path =
        "D:/Inventyv/Tasks/Training-2024-25/Rust/Rust Learning/error_handling/src/demo.txt";
    let myfile = File::open(file_path);

    let ans: File = match myfile {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(file) => {
                    println!("Created successsfully!!");
                    file
                },
                Err(e) => panic!("We can't create to file {e:?}"),
            },
            other_error => {
                panic!("Problem occur during to open file!!:{other_error:?}")
            }
        },
    };
    println!("File path is:{:?}", ans);
}
