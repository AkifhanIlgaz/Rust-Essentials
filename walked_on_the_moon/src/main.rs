use std::env;
use std::fs;
use std::io::prelude::*;
/*
    cargo run moonwalkers.txt <search name>
*/
fn main() {
    if env::args().len() < 2 {
        println!("Program requires two arguments: <file path> <search name>");
        return;
    }

    let path = env::args().nth(1).unwrap();
    let name = env::args().nth(2).unwrap();

    let moonwalkers = fs::read_to_string(path).unwrap();

    for (line, _name) in moonwalkers.lines().enumerate() {
        if _name == name {
            println!(
                "{name} walked on the moon. Check line {} in the file",
                line + 1
            );
            return;
        }
    }

    println!("{name} didn't walk on the moon");
}
