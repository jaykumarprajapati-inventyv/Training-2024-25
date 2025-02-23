use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn sending_multiple_value_receiver_wait_for_it() {
    let (sx, rx) = mpsc::channel();

    thread::spawn(move || {
        let v1 = vec![
            String::from("Jay"),
            String::from("Prajapati"),
            String::from("Ahmedabad"),
        ];

        for elem in v1 {
            sx.send(elem).expect("Failed at sending time!!");
            thread::sleep(Duration::from_millis(1000));
        }
    });

    
    for received in rx {
        println!("Values is:{received}");
    }
}
