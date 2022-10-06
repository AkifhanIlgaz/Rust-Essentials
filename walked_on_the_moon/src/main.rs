use std::env;
use std::fs;
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
    println!("path: {path}, name: {name}");

    let moonwalkers = fs::read_to_string(path).unwrap();

    for _name in moonwalkers.lines() {
        if _name == name {
            println!("{name} walked on the moon");
            return;
        }
    }

    println!("{name} didn't walk on the moon");
}
