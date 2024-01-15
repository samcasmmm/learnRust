use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("./src/demo.txt")?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Convert the Vec<u8> to a String
    let contents_string = String::from_utf8_lossy(&contents);

    println!("File contents: {}", contents_string);

    Ok(())
}
