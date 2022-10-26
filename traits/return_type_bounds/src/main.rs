use std::fmt;

// Return data type must implement Display trait

fn get_displayable(choice: bool) -> impl fmt::Display {
    // Error
    // Possible return types from an if else must be the same data type
    if choice {
        13
    } else {
        "Thirteen"
    }
}

fn main() {
    println!("{}", get_displayable(true));
    println!("{}", get_displayable(false));
}
