use std::io;

fn main() {
    let mut input = String::new();
    let mut value = String::new();

    println!("sin, cos, or tan?");
    println!("Enter -> 's' ... sin\nEnter -> 'c' ... cos\nEnter -> 't' ... tan");
    print!("> ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_lowercase();

    println!("Enter value(rad)");
    print!("> ");
    io::stdin().read_line(&mut value).expect("Failed to read line");
    let value: f64 = value.trim().parse().expect("Please enter a valid number");

    match input.as_str() {
        "s" => println!("sin({:.2}) = {:.2}", value, value.sin()),
        "c" => println!("cos({:.2}) = {:.2}", value, value.cos()),
        "t" => println!("tan({:.2}) = {:.2}", value, value.tan()),
        _ => println!("Invalid choice"),
    }
}