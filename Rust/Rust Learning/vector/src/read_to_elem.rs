/*Note:- (1)If you want to read elem. which is "String" type,we can't assign value. directly in vari. we've to pass "reference" of it. Because String type can't use "copy trait".
(2)But if we're assigning elem. that is num,character or string literal(&str) type then we can assign elem. bec. they use "copy trait".
 */

//There're two type to read elem:- (a)via index (b)get().
pub fn reading_elem() {
    let mut nums = vec![100, 140, 200, 130, 500];
    let no = nums[2]; //we can assign bec. it pass just copy.
    println!("Elem on 2nd index is:{}", no);
    /*
    Aa e.x. Rust site ma 8.6 example nu chhe.
    Aa nicheno code err. throw kare bec. apde upar nums na vector ni index2 ni val. no refer. apyo chhe,hve te "no" ae te reference ne point karse, to pasi nichni line ma apde push karta te old point ae invalid thase, to atle mate te mutable aek vae point karya pasi
    karvanu na kahe chhe.
      nums.push(15);
      println!("Elem on 2nd index is:{}", no);
     */

    println!("Nums is:{:?}", nums);

    let fullname = vec![String::from("Jay"), String::from("Prajapati")];
    let fname = &fullname[0]; //Not using "&" then throws err.
    println!("On index 0 value is:{}", fname);

    nums.push(1000);
    println!("All elem.:-{:?}", nums);

    //panic:-bec. try to access out of index val.
    // let out_of_index=nums[15];
    // println!("Out of index val. in num vec:-{}",out_of_index);

    //Get()
    let rollno = vec![5054, 5060, 5075, 5090];
    let rno = rollno.get(10);
    match rno {
        Some(no) => println!("Roll No is:{}", no),
        None => println!("You're trying to access out of index val."),
    }
}
