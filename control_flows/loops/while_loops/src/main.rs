fn main() {
    /*
        While loops will continue to execute if condition is true

        We can also use "break" and "continue" keywords as we did in the "loops" but while loops can't return a value
    */

    let mut number = 3;

    // Check the condition, if it is true run the code in the while block
    // Repeat this until the condition is false
    while number != 0 {
        println!("Number: {number}");
        number -= 1;
    }
    println!("Liftoff!!");
}
