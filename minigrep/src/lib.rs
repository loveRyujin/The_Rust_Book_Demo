use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, String> {
        if args.len() < 3 {
            return Err(String::from("not enough argument"));
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self { query: query, file_path: file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}