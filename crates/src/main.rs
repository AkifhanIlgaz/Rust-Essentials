use rand::prelude::*;
fn main() {
    /*
        Crates are collection of Rust source code files and can be divided into two parts:

            Binary Crates => Executable of your program, main.rs
            Library Crates => Contains code for other programs to use

        We will talk library crates.

        Basically, crates are NPM packages in Rust.

        You can visit crates.io to explore crates in Rust.

        Let's see where and how we can use crates.

        Rust doesn't have built-in random generator. In order to do that we will use "rand" crate.

        Cargo will build our dependencies before we run our program but we have to explicitly specify that which crates must be installed.

        Old Way
            Go to Cargo.toml file
            [dependencies]
            rand(<crateName>) = "0.8.5"(<version>)
        When we type "cargo run", cargo will build rand crate for us

        New Way
            We can install crates as we did with NPM. npm install packageName
            Go to terminal
            "cargo add crateName"
            Cargo will handle rest of it.

        In order to use a crate we have installed we have to tell the Rust compiler we want to use that crate

            use rand;

        Now, let's generate a random number by using "rand" crate
    */

    // We will use turbofish operator to specify the data type of random_number
    let random_number = rand::random::<i32>();
    println!("The random number is: {}", random_number);

    // If we use only one function from a crate or, use a specific function multiple times, we don't have to specify full path every time.
    //   use rand::random;

    let random = random::<i32>();
    println!("The random number is: {}", random);

    /*
        It is more readable but can cause ambiguity.
        Let's say we have a function named random()

        Now, Rust compiler will throw an error because there are two functions with the same name. Compiler doesn't know which one to call
    */

    /*
        There might be some crates that have some functions that are very purpose-specific and we don't want to use them.
        We only want to use general functions.

        In order to do that we can use prelude module of our crate. It includes most-used functions of this crate. And in order to use all functions of prelude we will use wildcard operator

        use rand::prelude::* => use rand::random  , use rand::thread_rng

        Instead of specifying all functions we want to use we can specify prelude
    */

    // rand::thread_rng().gen_range(0..11)
    let number = thread_rng().gen_range(0..11);
    println!("The number is {number}");
}

fn random() -> i32 {
    1
}
