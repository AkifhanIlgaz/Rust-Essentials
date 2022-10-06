use std::io;
fn main() {
    /*
        Rust Standard Library

        Rust standard library has a lot of modules that a Rust developer needs. Since we don't need all modules in the standard library in one project, we don't need to use them all. Rust installs some module that a developer definitely uses when writing a Rust program.

        If you need a module that is not automatically installed, you must specify the path of that module

        Let's see how we can use io module from standard library

        use std::io

        We explicitly tell that we want to use io module and now, compiler knows where it can find it

        Let's look at this example
    */

    let mut buffer = String::new();
    println!("Enter a message: ");
    io::stdin().read_line(&mut buffer); // We use stdin(),read_line() functions from io module
                                        // It takes an input from the user and updates the buffer

    println!("Your message is: {buffer}");

    /*
        There is no problem, if we will work with strings but what if we want to work with numeric values
        There is no way to take an input from the user as an integer. All input values are string
        We need to convert string input into integer
        In order to do that we will use parse()
    */

    // We need trim() method. Because the input we take from user includes new line character at the end
    // When we use parse() method, we have to specify the data type we want to convert into
    // There are 2 ways to do that => Turbofish | Explicit
    // We have used unwrap() method. It basically unwraps the result enum which we will talk it about in Error Handling

    // Turbofish operator
    let number = buffer.trim().parse::<i32>().unwrap();

    // Explicit data type
    let number: i32 = buffer.trim().parse().unwrap();

    println!("Number + 1 is : {:?}", number + 1);
}
