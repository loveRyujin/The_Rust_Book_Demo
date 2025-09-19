use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // normal_error_handling();
    concise_error_handling();
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

// func return_error() -> Result<File, std::io::Error> {
//     let file = File::open("hello.txt")?;
// }