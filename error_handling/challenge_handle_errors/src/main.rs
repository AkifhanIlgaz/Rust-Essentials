use io::Error;
use rand::prelude::*;
use std::io;
fn main() {
    let secret_number = rand::thread_rng().gen_range(0..101);

    println!("I'm thinking of a number between 0 and 100...");
    println!("Guess a number");

    loop {
        let mut buffer = String::new();

        let guess = match io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.trim().parse::<u32>() {
                Ok(value) => value,
                Err(_) => {
                    println!("\nFailed to parse input. Try again.");
                    continue;
                }
            },

            Err(_) => {
                println!("\nFailed to read input. Guess again.");
                continue;
            }
        };
        if guess > secret_number {
            println!("\n{} is too high! Guess lower:", guess);
        } else if guess < secret_number {
            println!("\n{} is too low! Guess higher:", guess);
        } else {
            println!("\nYou got it! The secret number was: {}", secret_number);
            break;
        }
    }
}
