use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("demo.txt")?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    println!("File contents: {:?}", contents);

    Ok(())
}