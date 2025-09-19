use std::fs::File;

fn main() {
    let result = File::open("hello.txt");

    let _file = match result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {error:?}")
        },
    };
}
