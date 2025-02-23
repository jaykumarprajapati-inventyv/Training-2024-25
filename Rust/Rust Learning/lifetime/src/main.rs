mod solu1_by_static;
mod solu2_lifetime;
fn main() {
    // let dangle_ref=dangle();
    let sol1 = solu1_by_static::solution1_by_static();
    println!("Val. by using static is:{sol1}");

    let s1 = String::from("Prajapati");
    let s2 = String::from("Jay");

    let sol2 = solu2_lifetime::solution2_by_lifetime(&s1, &s2);
    println!("Longest word is {sol2}");
}
/*
> It'll throw an err., bec. we can't pass reference of any val. outside of the func. Bec. that value's refe. will be erase after of that func. It known as "Dangling reference".

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

Sol. 1:- Just pass Ownership not reference.
Sol. 2:- Use "lifetime".
Sol. 3:- Pass reference through "'static".
*/
