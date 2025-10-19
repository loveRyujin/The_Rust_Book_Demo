use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let msg = String::from("Hi");
        tx.send(msg).unwrap();
    });

    let recieved = rx.recv().unwrap();
    println!("Got: {}", recieved);
}
