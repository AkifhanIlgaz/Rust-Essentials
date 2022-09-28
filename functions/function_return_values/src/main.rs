fn main() {
    /*
        We don't name return values but we have to specify its type

        "->" is used to specify return type
    */

    // Final expression in the function body is return value
    fn multiply_with_2(number: i32) -> i32 {
        number * 2
    }

    // Also, you can use "return" keyword if your function logic
    fn multiply_with_3(number: i32) -> i32 {
        if number >= 10 {
            return number;
        }
        return number * 3;
    }
}
