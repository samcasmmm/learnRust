use std::io;

fn read_input() -> String {
    let mut input = String::new();
    println!("Enter a message:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn output_value(value: &str) {
    println!("Input value: {}", value);
}

fn main() {
    let t = read_input();
    output_value(&t);
}
