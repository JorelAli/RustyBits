use std::fs::File;
use std::io::Result;
use std::io::prelude::Read;
// use std::io::prelude::*;

fn main() {
    println!("Hello, world!");
    let file_contents = print_file();

    println!("{}", file_contents.unwrap_or("Couldn't find file".to_string()));

    // if attempt_print.is_err() {
    //     println!("{:#?}", attempt_print.err());
    // } else {
    //     println!("{}", attempt_print.unwrap());
    // }
}

// Rust uses Result<> instead of exceptions and throws
fn print_file() -> Result<String> {
    // Prints current working directory
    println!("{:?}", std::env::current_dir().unwrap());

    // Opens a file
    let mut file: File = File::open("file.txt")?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
