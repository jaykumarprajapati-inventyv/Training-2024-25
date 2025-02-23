use std::{thread, time::Duration};

fn create_user() {
    let mut user = 0;
    user = user + 1;
    println!("User {user}");
}

fn main() {
    thread::spawn(|| loop {
        create_user();
        thread::sleep(Duration::from_secs(30));
    });

    thread::sleep(Duration::from_millis(10000));
}
