fn main() {
    /*
        Tuples are used to group multiple variables that can have different data types

        Tuples are fixed-size; once declared, they cannot grow or shrink in size

    */

    let tuple: (i32, f64, bool, char) = (450, 6.4, true, 'a'); // Don't need type annotations

    // Destructuring
    let (x, y, z, q) = tuple;
    // We breaked the tuple into four parts, and each part are assigned to the variable
    println!("x: {x}"); // 450
    println!("y: {y}"); // 6.4
    println!("z: {z}"); // true
    println!("q: {q}"); // a

    // If you need to destructure but don't want all variables use "_"
    let (_, _, myValue, _) = tuple;
    println!("{myValue}");

    // We can access tuple elements by dot notation
    // tuple.<index>
    let first_element = tuple.0;
    println!("The first element of the tuple: {}", first_element);

    /*
        The tuples without any values is called (unit).

        () represents empty value, empty return type
    */

    // do_something function gets empty tuple(unit)(nothing) as a parameter and return nothing(empty tuple)
    fn do_something() {
        println!("blablabla");
    }
    fn do_another_thing() -> () {}

    let unit = do_another_thing(); // The type of the "unit" variable is ()
}
