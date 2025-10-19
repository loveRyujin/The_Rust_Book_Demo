use std::{sync::{Mutex, Arc}, thread, sync::atomic};

fn main() {
    let shared_num = Arc::new(Mutex::new(0));
    let mut handlers = vec![];
    
    // Mutex
    for i in 1..=10 {
        let shared_num = Arc::clone(&shared_num);
        let handler = thread::spawn(move || {
            let mut num = shared_num.lock().unwrap();
            *num += 1;
        });

        println!("This is {i}th thread handler");
        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    println!("shared_num is {}", *shared_num.lock().unwrap());

    // AtomicI32
    let counter = Arc::new(atomic::AtomicI32::new(0));
    let mut handlers = vec![];

    for i in 1..=10 {
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            counter.fetch_add(1, atomic::Ordering::SeqCst);
        });
        
        println!("This is {i}th thread");
        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    println!("counter is {}", counter.load(atomic::Ordering::SeqCst));
}
