fn main() {
    /*
        Statements
         Perform an action without returning a value
         End with semicolon
    */

    let y = 5; // Statement
    let x = ( let y = 6 ); // Since statements don't return values you can't assign a "let" statement to a variable

    // Function definition is statement
    fn statement_function() {
        println!("..");
    }

    /*
        Expressions
         Returns a value
         Does not end with semicolon
    */

    let z = 3 + 7;
    /*
        3 + 7 => Expression, so returns a value
        3 + 7 returns 10
        10 is assigned to "z"
        let z = 10 => Statement
    */

    // Function call is expression and returns a value
    let function_call = statement_function(); // statement_function() will return "()"

    // A new scope block created with curly braces is an expression
    let scope_block = {
        let x = 3;
        x + 1 // It's the return value. If you add semicolon to the end of this line this will be a statement and don't return anything
    }; // We add semicolon here because we're assigning the value returned from scope block is assigned to scope_block variable
}
