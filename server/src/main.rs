use std::net::TcpListener;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut i = 1;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("incoming request{}!", i);
        i += 1;
    }
}
