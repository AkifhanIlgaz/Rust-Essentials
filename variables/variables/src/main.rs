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

    /*
        We can specify the data type of the variable.
        Check data_types for further information about types
    */

    let number: i8 = 5;
    let number = 1i8;
}
