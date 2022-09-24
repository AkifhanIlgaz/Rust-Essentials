fn main() {
    /*
        We use "let" keyword to declare a variable

        Variables are immutable by default

        We use snake_case for variables by convention
    */

    let snake_case = 5;
    println!("{snake_case}");
    snake_case = 6; // Since variables are immutable this line will throw an error

    // If you want to make a variable immutable , you should use "mut" keyword

    let mut y = 5;
    println!("{y}");
    y = 6; // No error
}
