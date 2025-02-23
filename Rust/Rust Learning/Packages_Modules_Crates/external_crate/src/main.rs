use rand::Rng; //external
fn main() {
    // let no = rand::thread_rng().gen_range(1..=100);
    let no = rand::rng().random_range(1..=100);
    println!("Random num is:{}", no);
}
