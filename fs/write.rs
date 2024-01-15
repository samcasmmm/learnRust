use std::fs::File;
use std::io::{self, Write};

fn write_to_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    // Write the content to the file
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn main() {
    let file_path = "example.txt";
    let content_to_write = "Hello, Rust!";

    // Attempt to write to the file
    match write_to_file(&file_path, content_to_write) {
        Ok(_) => println!("File written successfully"),
        Err(e) => eprintln!("Error writing to file: {}", e),
    }
}
