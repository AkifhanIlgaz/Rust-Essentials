fn main() {
    /*
        In Rust, you can declare a new variable with a name of an existing variable and it's called shadowing

        You can shadow a variable by declaring a new variable with "let" keyword with an existing name
    */

    let x = 5; // x is 5

    let x = x + 1; // x is 6 now. New "x" variable shadowed the old "x" variable

    {
        let x = x * 2; // x is 12 for this inner-scope
        println!("The value of x for inner-scope: {x}"); // 12
    }

    println!("The value of x: {x}"); //6

    /*
        We can change the data type of a variable by shadowing since we are creating a new variable

    */

    let boolean = 1; // i32
    let boolean = true; // bool

    /*
        We cannot do this mutable variables.Because we are not creating a new variable.

        When we are creating a variable Rust compiler must know the data type of it. Check the code below.
    */

    let mut y = 3; // Now, Rust compiler knows that y is a mutable variable which should hold an integer(i32) value

    y = String::from("THREE");
    /*
    Rust compiler expects that variable y should be binded to an integer value but we are trying to change it to String type
    Rust compiler will throw an error on line 35 =>     "expected i32, but found String"
    */
}
