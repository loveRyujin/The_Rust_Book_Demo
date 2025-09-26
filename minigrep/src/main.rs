use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("Problems parsing arguments: {err}!");
        process::exit(1);
    });

    println!("Searching for {}", cfg.query);
    println!("In file {}", cfg.file_path);

    if let Err(e) = minigrep::run(cfg) {
        println!("Application run error: {e}!");
        process::exit(1);
    }
}