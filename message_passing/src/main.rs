use std::{thread, vec};
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let msg = String::from("Hi");
        tx.send(msg).unwrap();
    });

    let recieved = rx.recv().unwrap();
    println!("Got: {}", recieved);


    let (tx, rx) = mpsc::channel();

    let txc = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("World"),
        ];

        for val in vals {
            txc.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("imformation"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved in rx {
        println!("Got: {}", recieved);
    }
}
