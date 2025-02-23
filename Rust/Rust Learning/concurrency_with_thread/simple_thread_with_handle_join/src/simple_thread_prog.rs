use std::thread;
use std::time::Duration;

pub fn simple_thread_program() {
    //spawn thread
    thread::spawn(|| {
        for i in 1..10 {
            println!("I'm from {i} simple and spawn thrad!!");
            thread::sleep(Duration::from_millis(1000));
        }
    });

    //main thread
    for i in 1..5 {
        println!("I'm from {i} main thread.");
        thread::sleep(Duration::from_millis(1000));
    }
}
