fn main() {
    /*
        Passing a value to a function are similar to when assigning value to a variable

        Passing a value will move or copy, like what assignment does
    */

    let message = String::from("Hello"); // s comes into scope

    takes_ownership(message); // message's value moves into the function, so it's no longer valid in this scope
    println!("The message is: {message}"); // Error

    let x = 5; // x comes into scope

    makes_copy(x); // x's value is copied and passed to the function, so x is still valid

    println!("The number is: {x}"); // OK
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    //
    println!("{some_string}");
} // some_string goes out of scope and "drop" is called. Memory is freed

fn makes_copy(number: i32) {
    // number comes into scope
    println!("{number}");
} // number goes out of scope. It was stored on the stack, therefore nothing special happens
