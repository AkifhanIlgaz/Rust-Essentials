fn main() {
    /*
        Returning values can also transfer ownership. If we bind the return value to a variable, new variable takes the ownership of the return value
    */

    let s1 = gives_ownership(); // gives_ownership moves its return value to s1

    let s2 = String::from("Hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, and its return value is moved to s3

    /*
        Taking ownership and returning it works fine, but it's tedious.Let's look at this example
    */

    let s1 = String::from("Hello");

    let (s2, len) = calculate_length(s1);

    // Here, all we want to do is calculating the length of a string but we also have to return the ownership of the parameter. Imagine if we have to do that for every function even if it is very easy and basic
    // Fortunately, Rust has a feature that enables us to take values as parameter without taking ownership of it, called "references";
}

fn calculate_length(string: String) -> (String, usize) {
    let length = string.len();
    (string, length)
}

// gives_ownership() will move its return value into the function that calls it
fn gives_ownership() -> String {
    let some_string = String::from("Earth"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

// Takes a String and returns it
fn takes_and_gives_back(s: String) -> String {
    // s comes into scope
    s // s is returned and moves out to the calling function
}
