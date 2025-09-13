use std::io;

fn main() {
    let mut a = 1;
    let mut b = 1;

    let mut n = String::new();
    println!("Enter the number of terms: ");
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = n.trim().parse().expect("Failed to parse integer");

    if n == 1 || n == 2 {
        println!("The {n}th term is 1");
        return;
    }

    let mut c: u32 = 0;
    for _ in 2..n {
        c = a + b;
        a = b;
        b = c;
    }

    println!("The {n}th term is {c}");
}
