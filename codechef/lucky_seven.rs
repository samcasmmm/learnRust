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
    let x = read_string();
    if let Some(seven) = x.chars().nth(6) {
        println!("{}", seven);
    } else {
        println!("No character found at index 7");
    }
}
