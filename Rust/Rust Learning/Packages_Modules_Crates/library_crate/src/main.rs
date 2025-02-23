/*If we want to use library crate's code inside in main.rs(Binary crate), then we need to use our first package name same as below.
*/
fn main() {
    let ans = library_crate::my_lib_module::sum(12, 2);
    println!("Ans is:{}", ans);
}
