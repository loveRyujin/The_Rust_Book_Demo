use std::thread;
use std::time::Duration;

fn main() {
    thread_process();
    thread_process_with_move();
}

fn thread_process() {
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handler.join().unwrap();
}

fn thread_process_with_move() {
    let v = vec![1, 2, 3];

    let handler = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handler.join().unwrap();
}
