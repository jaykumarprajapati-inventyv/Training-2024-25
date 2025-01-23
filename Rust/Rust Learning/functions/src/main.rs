fn main() {
    println!("Hello, world!");
    first_func("Jay", 'J');

    return_expression_func_1();

    //Return expresion after taking parameter
    let x = return_expression_func_2(5);
    println!("The value of x is: {x}");

    early_return_func(5);
}

//Func with paramter
//Note:-Parameter ne type apvo jaruri chhe.

fn first_func(name: &str, unit_label: char) {
    //String ma j hovu joie
    println!("I'm from first_func {} & {}", name, unit_label)
}

//Function return expression:-Ahi 'y' ae return thayel x+1 ne le chhe.
//Imp note:- return part na end ma ";" na hoi,nahi to error aape.
fn return_expression_func_1() {
    let y = {
        let x = 5;
        x + 1
    };
    //Semicolon jaruri chhe.
    println!("The value of y is: {y}");
}

//Return expresion after taking parameter
fn return_expression_func_2(no: i32) -> i32 {
    no * 2
}

fn early_return_func(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    println!("num is not even");
    false
}
