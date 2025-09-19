use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // normal_error_handling();
    // concise_error_handling();
    match return_error() {
        Ok(file) => println!("[one] File opened successfully"),
        Err(e) => println!("[one] Problem opening the file: {e:?}"),
    }
    match return_error_2() {
        Ok(file) => println!("[two] File opened successfully"),
        Err(e) => println!("[two] Problem opening the file: {e:?}"),
    }
}

fn normal_error_handling() {
    let result = File::open("hello.txt");

    let _file = match result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {error:?}")
        },
    };
}

fn concise_error_handling() {
    let result = File::open("hello.txt");

    let _file = match result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                    println!("File not found");
                    File::create("hello.txt").expect("Failed to create file")
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}")
            },
        }
    };
}

fn return_error() -> Result<File, std::io::Error> {
    let file = File::open("hello.txt")?;
    Ok(file)
}

fn return_error_2() -> Result<File, std::io::Error> {
    File::open("hello.txt")
}