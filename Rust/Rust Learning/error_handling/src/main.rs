mod matching_with_diff_err;
mod propagating_the_error;
mod propagating_the_error_by_shortcut_way;

/* File::open(file_path):- It return Result<File,io::Error>, for "File" it'll return "OK" and for "io::Error" will return "Err". */
fn main() {
    //If file isn't exist, then we'll create the file so by this way we're handling to error.
    // matching_with_diff_err::matching_with_diff_err();

    // let inner_content = propagating_the_error::propagating_the_error();
    // println!("Inner content is:{:?}", inner_content.unwrap());

    let ans=propagating_the_error_by_shortcut_way::propagating_the_error_by_shortcut_way();
    println!("Ans of shortcut way is:{:?}",ans.unwrap());
}
