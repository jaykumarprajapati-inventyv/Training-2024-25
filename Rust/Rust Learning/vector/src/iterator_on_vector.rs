/*Note:- If we're not write "&" ahead of v1 then it'll be give ownership to "i",then we can't access or mutate to "v1". By  &v1 means borrowing (read-only access), so v1 remains usable. */

pub fn iterate_on_vector() {
    let mut v1 = vec![12, 5, 89, 32];
    println!("Iterate using for loop");
    for i in &v1 {
        println!("{i}")
    }


    //Mutate
    println!("Mutate to element using for loop");
    for i in &mut v1 {
        *i += 1;
        println!("{i}")
    }
}
