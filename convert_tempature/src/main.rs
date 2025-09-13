use std::io;

fn main() {
    println!("Convert temperature");

    let mut c_tof = String::new();

    println!("Enter yes to convert Celsius to Fahrenheit, no to convert Fahrenheit to Celsius");
    io::stdin().read_line(&mut c_tof).expect("Failed to read line");

    let flag: bool = if c_tof.trim() == "yes" { true } else { false };

    match flag {
        true => {
            let mut celsius = String::new();
            println!("Enter the temperature in Celsius");
            io::stdin().read_line(&mut celsius).expect("Failed to read line");

            let celsius: f64 = celsius.trim().parse().expect("Failed to parse float");
            let fahrenheit = (celsius * 1.8) + 32.0;
            println!("{} degrees Celsius is {} degrees Fahrenheit", celsius, fahrenheit);
        }
        false => {
            let mut fahrenheit = String::new();
            println!("Enter the temperature in Fahrenheit");
            io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");

            let fahrenheit: f64 = fahrenheit.trim().parse().expect("Failed to parse float");
            let celsius = (fahrenheit - 32.0) / 1.8;
            println!("{} degrees Fahrenheit is {} degrees Celsius", fahrenheit, celsius);
        }
    }
}
