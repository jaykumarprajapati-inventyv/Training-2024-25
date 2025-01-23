mod give_ownership;
mod makes_copy;
mod return_through_tuple;
mod takes_and_gives_back_ownership;
mod takes_ownership;
fn main() {
    /*Note:-String ma ownership badlay,jyare int ma nahi,bec. aene fixed size and ae stack no use kare chhe. */

    //Ahi 's' ni ownership ae "takes_ownership" ae lidhi.
    // println!("Hello, world!");
    let s = String::from("Hello, world!");
    takes_ownership::takes_ownership(s);

    /*Aa niche nu print ae error aapse,bec. 's' ni ownership ae "takes_ownership" pase gai atle, 's' ae memory thi free thayu. */
    // println!("Takes ownership through original:-{s}");

    let no = 15;

    makes_copy::make_copy(no);
    //Aa niche nu error nahi aape bec. ahi aeni copy pass thay chhe.
    println!("Makes copy through original:-{no}");

    //** Gives ownership **/
    //Ahi func. ma return val. ni ownership "name" ne aapi.
    let name = give_ownership::give_ownership();
    println!("Name from give_ownership:-{name}");

    //** Takes & Gives back ownership:-Ahi surname ni api,sname ma lidhi. **/
    let surname = String::from("Prajapati");
    let sname = takes_and_gives_back_ownership::takes_and_gives_back_ownership(surname);
    println!("Surname from takes_and_gives_back_ownership:-{sname}");

    let address = String::from("Ahmedabad");

    let (addr, length) = return_through_tuple::return_through_tuple(address);
    println!("Address={addr} and length={length}.");
}
