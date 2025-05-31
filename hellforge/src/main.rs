use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Enter value: ");
    let mut input = String::new();
    let comp = 5;

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input!");

    let input: u32 = input.trim().parse().expect("Enter a number!");

    match input.cmp(&comp) {
        Ordering::Less => println!("Input is Less!"),
        Ordering::Greater => println!("Input is Greater!"),
        Ordering::Equal => println!("Right!"),
    }
}
