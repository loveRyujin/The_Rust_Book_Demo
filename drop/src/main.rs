struct CustomStruct(String);

impl Drop for CustomStruct {
    fn drop(&mut self) {
        println!("Dropping custom struct with data: {}", self.0);
    }
}

fn main() {
    let s1 = CustomStruct(String::from("s1"));
    let s2 = CustomStruct(String::from("s2"));
    println!("s1 and s2 created successfully!");
    drop(s1);
    println!("before the end of main function.")
}
