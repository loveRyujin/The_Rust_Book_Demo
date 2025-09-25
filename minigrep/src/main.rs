use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("Problems parsing arguments: {err}!");
        process::exit(1);
    });

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
    fn new(args: &[String]) -> Result<Self, String> {
        if args.len() < 3 {
            return Err(String::from("not enough argument"));
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self { query: query, file_path: file_path })
    }
}