use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Config::new(&args);

    println!("Searching for {}", cfg.query);
    println!("In file {}", cfg.file_path);

    let contents = fs::read_to_string(cfg.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Self { query: query, file_path: file_path }
    }
}