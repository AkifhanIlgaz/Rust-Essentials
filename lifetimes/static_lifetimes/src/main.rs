/*
    'static Lifetime

    Indicates references that is available for entire duration of the program

    Example: string literals
    Since string literals are stored on the executable, they will never go out of scope

        let s: 'static str = "Greetings from Neptune"

    Can be coerced to more restrictive lifetime

    'static Lifetime as a Trait Bound

    Ensures data type will only contain 'static elements
        <T: Display + 'static>
        T does not contain any non-static references
*/

fn main() {}
