fn main() {
    /*
        If expressions are used to decide how program will run depending on conditions
    */

    let number = 5;

    if number > 15 {
        println!("{number} is greater than 15");
    } else {
        println!("{number} is smaller than 15");
    }

    // The condition of the if expression must be bool !!
    // This if expression will throw an error since the condition isn't a boolean value
    let condition = 1;
    if condition {
        println!("The condition is true");
    } else {
        println!("The condition is false")
    }

    // if, else if and else expressions are useful if there are multiple conditions that you wanna check
    let num = 15;
    if num % 2 == 0 {
        println!("Number is divisible by 2")
    } else if num % 3 == 0 {
        println!("Number is divisible by 3")
    } else {
        println!("Number is not divisible by 2 and 3");
    }
    // A lot of else if expressions will make complex your code.Instead of doing that you can use "match" expression

    // Since if is an expression we can assign the return value to a variable
    // But possible return values must be same data type.Let's look at this example

    let condition = true;
    let result = if condition { 5 } else { "Five" };
    /*
    Rust must know the data type of a variable at compile time but there are two possible return values of if expression.
    We can only know the return data type of if expression at runtime.So our example will throw error
    */
}
