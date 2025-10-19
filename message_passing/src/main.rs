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


    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("World"),
        ];

        for val in vals {
            tx.send(val).unwrap();
        }
    });

    for recieved in rx {
        println!("Got: {}", recieved);
    }
}
