#[derive(Debug)]
struct User {
    username: String,
    gender: char,
    mobileno: u64,
    married: bool,
    salary: f64,
}

fn main() {
    //By default immutable instance hoi chhe, jeni field ni val. ne change karva "mut" use karyu.
    let mut user1 = User {
        username: String::from("Jay"),
        gender: 'M',
        mobileno: 9375221221,
        married: true,
        salary: 250000.550,
    };
    user1.username = String::from("Jaykumar");

    println!("Username is:{}", user1.username);
    println!("Salary of {} is:{}", user1.username, user1.salary);

    // let user_name = String::from("Jay2833");
    // let user2 = build_user(user_name);
    //or
    let user2 = build_user(user1.username);

    println!("User2 through username is:{}", user2.username);

    println!("** User 3 object's fields initialize by using user1's field's value. **");

    let user3 = User {
        username: String::from("Anil"),
        gender: user1.gender,
        mobileno: user1.mobileno,
        married: false,
        salary: user1.salary,
    };
    println!("Mobile no is:{}", user3.mobileno);

    //** Creating Instances from Other Instances with "Struct Update Syntax" **/
    println!("User1 no aakho obj. username sivay ae user 4 ma assign karyo using \"Struct Update Syntax\" ");
    let user4 = User {
        username: String::from("Pritesh"),
        ..user1
    };
    println!("Username of user 4 instance is:{}", user4.username);
    println!("salary of user 4 by user1's instance  is:{}", user1.salary); //Ahi user1 ni sal. access karay,bec. ae scalar type no fixed size ni val.chhe je stack ma store chhe atle.

    //Nicheni line err. aape,bec. username ae string ma hovathi ae heap ma store hovathi aeni ownership hve user2 pase gai atle.
    // println!("salary of user 4 by user1's instance  is:{}", user1.username);

    //** Struct ne debug karva niche ni line and upar nu #[derive(Debug)] */
    println!("Instance number user3's data:{user3:#?}");
}

//Imp note:-Struct ne je field name chhe,aej ahi parameter ma apvu,nahi to error aapse.
fn build_user(username: String) -> User {
    User {
        username, //or //   username: username
        gender: 'M',
        mobileno: 9375221221,
        married: true,
        salary: 250000.550,
    }
}
