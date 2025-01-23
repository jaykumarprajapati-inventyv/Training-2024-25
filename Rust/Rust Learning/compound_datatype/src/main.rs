fn main() {
    //Two type of compound:-(a)Tuple (b)Array

    //(A)Tuple
    let tup1: (i32, f64, bool, &str) = (100, 34.12, true, "Jay");
    println!("Tup1 by :#? is: {:#?}", tup1);
    println!("Tup1 by :? is: {:?}", tup1);

    //Destructuring
    let (a, b, c, d) = tup1;
    println!("Value of x is:{a}");

    //
    let e = tup1.2;
    println!("Value on 2nd index is:{}", e);

    //(B)Array
    let ar1 = [1, 2, 3, 4, 5];
    println!("Array's values are:{:?}", ar1);
    println!("Array's values are by using :#?:{:#?}", ar1);

    //Acces via index
    println!("Value on 2nd index:-{:?}", { ar1[2] });
    //Print elements for some time:- 5 8 var print karse.
    let ar2 = [5; 8];
    println!("Values of 2nd Array:{:?}", ar2)
}
