use std::collections::HashMap;
fn main() {
    let mut h1 = HashMap::new();

    h1.insert("Rank number", 3);
    h1.insert("Roll number", 5054);
    h1.insert("Mark", 90);
    h1.insert("Roll number", 5036); //Override to first

    /*  h1.insert("Gender", "Male"); // Throw error*/
    println!("{:?}", h1);

    //Accessing Values in a Hash Map
    println!("\n *** Accessing Values ***");
    let rank = h1.get("Rank number");
    println!("Your rank num is:- {:?}", rank);

    //or
    let roll_no = String::from("Roll number");
    let rno = h1.get(&roll_no.as_str());
    println!("Roll num is:-{:?}", rno);

    //or:- aa val. no reference return nahi kare,bec."copied()".
    let your_mark = h1.get("Mark").copied().unwrap_or(0);
    println!("Your mark is:{:?}", your_mark);

    println!("\n *** Iterate Values ***");
    for (key, val) in &h1 {
        println!("{}=>{}", key, val);
    }

    println!("*** Adding a Key and Value Only If a Key Isn’t Present ***");

    //Adding a Key and Value Only If a Key Isn’t Present
    h1.entry("sheet number").or_insert(455656);
    h1.entry("Mark").or_insert(80);
    println!("After inserting:-{:?}", h1);

    //Updating a Value Based on the Old Value
    println!("Updating a Value Based on the Old Value");
    let text = "hello world wonderful world";

    let mut res=HashMap::new();

    for word in text.split_whitespace(){
        
        let count=res.entry(word).or_insert(0);
        *count+=1;

    }
    println!("Word occurance:{:?}",res);

}

//alag dtype val
//rno vadhare to name pasi
