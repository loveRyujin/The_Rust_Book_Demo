use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let cfg = Config::build(env::args()).unwrap_or_else(|err_string| {
        eprintln!("Problems parsing arguments: {err_string}!");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(cfg) {
        eprintln!("Application run error: {e}!");
        process::exit(1);
    }
}