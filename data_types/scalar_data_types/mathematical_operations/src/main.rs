fn main() {
    /*
        Rust has all basic mathematical operations that you can use with integers and floating-point numbers.

        When you are doing mathematical operations the types of operands must be the same.
    */

    // Addition
    let add = 5 + 9;
    //let different_type_operand = 5 + 9.;
    /*
      Compiler will throw an error. Operands must be the same.
      What you are trying to do is i32 + f64
      In order to do that we will use type casting / explicit conversion. Check type_casting directory
    */

    // Subtraction
    let subtract = 7.8 - 7.7;

    // Multiplication
    let multiple = 4 * 30;

    /*
        We do exponentiations via pow() method, but there are few rules about it

        Rust must know the type of the base number. It needs to be explicitly annotated.

        You do this in 2 ways. Check the examples below.
    */

    // Exponentiation
    let base: i32 = 2;
    let exponent = base.pow(5);
    let exp = i32::pow(2, 5);

    // Division
    // Integer division rounds down to the nearest integer. Math.trunc() in Javascript
    let float_division = 25.2 / 3.2;
    let integer_division = 3 / 2; // 1

    // Modulo/Remainder

    let remainder = 10 % 2;
}
