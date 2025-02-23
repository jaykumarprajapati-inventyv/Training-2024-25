use std::sync::mpsc;
use std::thread;

pub fn simply_passing_val() {
    let (sx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msg = String::from("Hello world!!");
        sx.send(msg).expect("Failed at sending time!!");
        // println!("Message is:- {msg}"); It'll give an err. bec. ownership of msg is transferred to the receiver (rx).
    });

    let result = rx.recv().expect("Failed at receiving time.");
    println!("I'm main thread and i received {result} from spawn thread.");
}
