fn main() {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
        "eleventh", "twelfth",
    ];

    for i in 0..12 {
        println!("\nOn the {} day of Christmas,", days[i]);
        println!("My true love sent to me");

        for j in (0..=i).rev() {
            if j == 0 && i > 0 {
                print!("And ");
            }
            if j == 0 {
                println!("{}", gifts[j]);
            } else {
                println!("{},", gifts[j]);
            }
        }
    }
}