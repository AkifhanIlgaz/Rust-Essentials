use rand::prelude::*;
use std::io;
fn main() {
    let random_number = rand::thread_rng().gen_range(0..=100);
    let mut number_of_guesses_remaining = 5;
    loop {
        
        let mut guess = String::new();
        println!("Enter a number");
        io::stdin().read_line(&mut guess);
        let number = guess.trim().parse::<i32>().unwrap();
        number_of_guesses_remaining -= 1;
        if number < random_number {
            println!("Too low");
        } else if number > random_number {
            println!("Too high");
        } else {
            println!("Congrats!");
            break;
        }
        if number_of_guesses_remaining < 1 {
            println!("You have no guesses remaining");
            break;
        }
        println!("You have {} guesses remaining.", number_of_guesses_remaining);
    }
}
