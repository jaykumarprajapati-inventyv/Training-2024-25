fn main() {
    let no1 = 10;
    let no2 = 5;

    let res = addition(no1, no2);

    match res {
        Some(value) => {
            println!("Division is:{}", value);
        }
        None => {
            println!("You'r no2 is zero")
        }
    }

    problem_and_solution_program();
}
fn addition(n1: i32, n2: i32) -> Option<i32> {
    if n2 == 0 {
        None
    } else {
        Some(n1 / n2)
    }
}

fn problem_and_solution_program() {
    let x: u8 = 5;
    let y: Option<u8> = Some(10);

    //Nicheni comment line ae error nakhe because, u8+Option<u8> possible nathi
    // let ans=x+y;

    /* Solution(1):- unwrap() ae y ni val. ne extract kare chhe. Jo 'y' ni val. '0' hase to unwrap() panic errro nakhse. */
    let ans = x + y.unwrap();
    println!("Solution of Sum by unwrap():-{}", ans);

    /* Solution(2):-The match is safer as it explicitly handles the None case. */

    let res = match y {
        Some(value) => x + value, //Here, 'value' is Some's inner value which is 10.

        None => x, //Handle case of if y is None.
    };
    println!("Solution of Sum by match:-{}", res);
}
