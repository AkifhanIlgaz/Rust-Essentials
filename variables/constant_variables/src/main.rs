const ONE_BYTE_IN_BIT: i32 = 8;

fn main() {
    /*
        Constant variables is like normal variables we have explained.But there are few restrictions

        We're not allowed to use "mut" keyword,that means,constant variables are always immutable

        We use "const" keyword to declare a constant variable

        Data type of the constant variable must be annotated

        Constants can be declared in any scope, even outside of main() function

        Constants cannot be set the result of a value that could only be computed at runtime

        We use SCREAMING_SNAKE_CASE by convention
    */

    const ADDRESS_LENGTH_IN_BYTE: i32 = 20;

    println!("Address must be {} bytes", ADDRESS_LENGTH_IN_BYTE);

    println!("1 byte is {}", ONE_BYTE_IN_BIT);
}
