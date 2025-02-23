use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn multiple_thread_by_using_clone() {
    let (sx, rx) = mpsc::channel();

    let s2 = sx.clone();
    thread::spawn(move || {
        let v1 = vec![1, 2, 3, 4, 5];

        for elem in v1 {
            s2.send(elem).expect("Failed at sending time!!");
            thread::sleep(Duration::from_millis(1000));
        }
    });

    thread::spawn(move || {
        let v2 = vec![6, 7, 8, 9, 10];

        for elem in v2 {
            sx.send(elem).expect("Failed at sending time!!");
            thread::sleep(Duration::from_millis(1000));
        }
    });

    for received in rx {
        println!("Received element:{received}");
    }
}
