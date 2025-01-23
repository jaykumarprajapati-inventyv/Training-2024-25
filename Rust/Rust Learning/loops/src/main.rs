mod for_loop;
mod while_loop;
fn main() {
    //There are total "3" types of loop:-(a)loop (b)while (c)while

    //** Returning Values from Loops **
    let mut num = 0;
    //Ahi loop mathi return thatu hoi aene vari. ma store karvu j padse,nahi to ae o/p ma 10 aape.
    let ans = loop {
        num += 1;

        if num == 10 {
            break num * 2;
        }
    };
    println!("Num is:{ans}");

    //** Loop Labels to Disambiguate Between Multiple Loops **/
    //Jyare nested loop hoi tyare,loop ne label aapi shakai. Je label "single quote" ma hoi.

    /*Aa niche ni loop ae "remaining ae 9" thay tyare inner loop mathi bahar jase,jyare "count == 2" time ae
    aakhi loop atle outer loop mathi bahar jase.
    */
    let mut count = 0;

    'counting_up: loop {
        println!("Count is:{count}");
        let mut remaining = 10;
        loop {
            println!("Remaining is:{remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("** While loop **");
    while_loop::while_loops();

    println!("** For loop **");
    for_loop::for_loops();
}
