fn main() {
    //Updating a String
    let mut s1 = String::from("नमस्ते");
    s1.push_str(" in hindi");
    s1.push(',');
    println!("S1's val. is:{}", s1);

    //Covert String literal(&str) to String
    let s2 = "Jay".to_string();
    println!("Covert into String:-{}", s2);

    let concate = s1 + &s2;
    println!("After concated:{}", concate);

    let s3 = "Prajapati";
    let s = format!("{s2}*{s3}");
    println!("Formate through:-{}", s);

    //Methods for Iterating Over Strings

    let hindi = "नमस्ते";
    println!("By using char()");
    for ch in hindi.chars() {
        println!("{}", ch);
    }

    println!("By using bytes()");

    for bt in hindi.bytes() {
        println!("{}", bt);
    }

    //Ahi 0 to 3 atle,bec. hindi ae per char. ae '3 byte' le chhe.
    let s = &hindi[0..3]; //O/P:- न
    println!("Through range:- {}", s);
}
