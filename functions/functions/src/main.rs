fn main() {
    /*
        You must be familiar with functions since you have seen main() function, entry point of any program, even so, let's see how can we define a function

        fn function_name() {
            function body
        }

        We use snake_case for naming functions in Rust by convention
    */

    // Rust doesn't care where we have defined out functions, we can call any function that is defined in any scope that can be seen by caller

    some_function(); // We have defined this function below main(), but still we can call it

    /*
        Call a function with parameters

        The parameters of function_with_parameters() is x and y
        The arguments are 1 , 2

        "argument" and "parameter" words can be use interchangeably.
    */

    function_with_parameters(1, 2); //
}

fn some_function() {
    println!("some_function is called");
}

/*
    Function with parameters

    * You must declare the type of each parameter

*/

fn function_with_parameters(x: i32, y: i32) {
    println!("The parameters are {x} and {y}");
}
