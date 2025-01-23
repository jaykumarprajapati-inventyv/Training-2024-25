#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

/*If we want to ignore condi. which is not satisfy with our match,we can use as this:- _=>() or test(), other=>test(other).Here,other will be send unmatched val. to test().*/
fn main() {
    let coin_price = match_expression_1(Coin::Nickel);
    println!("Coin's value is:{}", coin_price);

    let city_name = match_expression_2(Coin::Quarter(UsState::Alabama));
    println!("Coin's value from city is:{}", city_name);

    if_let_expression();
}

fn match_expression_1(my_coin: Coin) -> u8 {
    match my_coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
        Coin::Dime => 10,
    }
}

fn match_expression_2(my_city: Coin) -> u8 {
    match my_city {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
        Coin::Dime => 10,
    }
}

/*By using "if let" we can ignore to write "_" for handle to "None" or "case which was not match".If we want to handle then we can do by else block."*/
fn if_let_expression() {
    let mut config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("By using if let:- {max}");
    } else {
        println!("config_max is None");
    }
}
