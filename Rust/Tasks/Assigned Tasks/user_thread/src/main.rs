use std::{
    sync::{Arc, RwLock},
    thread::{self},
    time::Duration,
};

#[derive(Debug)]
struct Users {
    id: i32,
    name: String,
}

fn main() {
    let users: Arc<RwLock<Vec<_>>> = Arc::new(RwLock::new(vec![]));
    let user_id_counter = Arc::new(RwLock::new(0));
    let user_id_counter_clone=user_id_counter.clone();
    

    //For threads
    let add_user_list_clone = users.clone();
    let total_len_of_users_clone = users.clone();
    let remove_to_user_clone = users.clone();
    let print_all_user_clone = users.clone();
    let stop_to_prog_clone = users.clone();

    let add_user = thread::spawn(move || loop {
        let mut id_counter = user_id_counter_clone.write().unwrap();
        *id_counter += 1;

        let user: Users = Users {
            id: *id_counter,
            name: String::from("Jay"),
        };

        add_user_list_clone
            .write()
            .expect("failed to insert")
            .push(user);
        thread::sleep(Duration::from_millis(500));
    });

    let print_length = thread::spawn(move || loop {
        println!(
            "Total users are:{}",
            total_len_of_users_clone.read().unwrap().len()
        );
        thread::sleep(Duration::from_millis(2000));
    });

    let remove_user = thread::spawn(move || loop {
        if remove_to_user_clone.read().unwrap().len() > 0 {
            remove_to_user_clone.write().unwrap().remove(0);
        }
        thread::sleep(Duration::from_millis(3000));
    });

    let print_all_users = thread::spawn(move || loop {
        println!("All users:{:?}", print_all_user_clone.read());
        thread::sleep(Duration::from_millis(3000));
    });

    let stop_program = thread::spawn(move || loop {
        if stop_to_prog_clone.read().unwrap().len() > 100 {
            panic!()
        } else {
            thread::sleep(Duration::from_millis(200));
        }
    });

    match stop_program.join() {
        Ok(_) => println!("We're exited from all threads!!"),
        Err(_) => println!("We crossed 100 length!!"),
    }
}
