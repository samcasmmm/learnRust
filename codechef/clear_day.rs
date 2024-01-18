use std::io;

fn read_ints() -> Vec<i32> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

fn main() {
    let input = read_ints();
    let res =  7 - input[0] - input[1];
    println!("{}",res); 
}
