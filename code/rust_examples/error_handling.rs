// Error handling with Result and Option

use std::fs::File;
use std::io::{self, Read};

fn main() {
    // Using match
    match read_file("test.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }

    // Using ? operator
    if let Err(e) = run() {
        println!("Application error: {}", e);
    }

    // Option type
    let numbers = vec![1, 2, 3];
    match numbers.get(1) {
        Some(n) => println!("Second element: {}", n),
        None => println!("No second element"),
    }
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let _file = File::open("nonexistent.txt")?;
    Ok(())
}
