fn main() {
    let num = 6;

    if num % 4 == 0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3 or 2!!");
    }

    //Using if in a let Statement
    let condition = true;
    let num = if condition { 5 } else { 6 };

    println!("Number is:{num}");

    /*Aa nicheno code error ape,bec. ae same type nu vari. ma assign nathi thatu,
     because if ae num. jyare else ae "string" aape chhe atle.
    */

    //let number = if condition { 5 } else { "six" };

    /*Aa nicheno code error ape,bec. condition ma always bool hovu joie jyare ahi
    num. chhe, ae nathi true ke false,ahi JS jevu work na kare.
    */
    // let number = 3;

    // if number {
    //     println!("number was three");
    // }
}
