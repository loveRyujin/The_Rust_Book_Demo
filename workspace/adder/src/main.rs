use add_one;
use add_two;

fn main() {
    let num = 10;
    println!("the result of add_one is {}", add_one::add_one(num));
    println!("the result of add_two is {}", add_two::add_two(num));
}
