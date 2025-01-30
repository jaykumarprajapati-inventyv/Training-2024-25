fn main() {
    /*Enum Option with Generics Type:- Some and None. */
    let num: Option<i32> = Some(5);
    let name: Option<i32> = None;

    check_val(num);
    check_val(name);

    /*Enum Result with Generics Type:- Ok and Err. */
    let num: Result<i32, String> = Ok(12);
    let err: Result<String, String> = Err(String::from("Invalid number"));

    result(num);
    result(err);
}
fn check_val<T: std::fmt::Display>(val: Option<T>) {
    match val {
        Some(val) => println!("Value is:{}", val),
        None => println!("No value found"),
    }
}

fn result<T, E>(val: Result<T, E>)
where
    T: std::fmt::Display,
    E: std::fmt::Display,
{
    match val {
        Ok(val) => println!("Value is:{}", val),
        Err(err) => println!("Error is:{}", err),
    }
}
