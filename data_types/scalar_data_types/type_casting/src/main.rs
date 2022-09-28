fn main() {
    /*
        As we see on the mathematical_operations we can use type casting when we are trying to do mathematical operations between the integers that have different types.

    */
    let i = 3 as f64;

    let x: i8 = 10;
    let y: i32 = 5;
    let error_result = x / y; // Error operand types must be same

    // Type casting
    let success_result = x as i32 / y;
    let success_result = x / y as i8;
    /*
        "x" is an i8 type variable.Instead of representing it in 8 bits we are representing it in 32 bits now.

        x => 00001010
        x as i32 => 00000000000000000000000000001010

    */

    // !! When you're type-casting some info might be lost and compiler won't throw an error

    let truncated = 3.9 as i32; // 3
    let modulo = 300 as u8; // 44

    /*
        3.9 => 3   Fractional information is lost

        300 => 44 Since 300 is bigger than max number that i8 type can store, some info is lost
            300 % max(i8) + 1 = 300 % 256 = 44
    */
}
