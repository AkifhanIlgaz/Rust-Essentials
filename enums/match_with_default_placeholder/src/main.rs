fn main() {
    let my_number = 5u8;

    /*
        match operator can be used to evaluate more than just enums.

        Return value of match can be assigned to a variable.

        Match expressions must be exhaustive which means that arms must be cover all possible values

        In our match expressions we want to return the string expression of my_number.

        my_number is u8, so there are 256 possible values. We need to cover them all. Imagine if my_number is i64.

        Instead of typing out all possible values we will use default placeholder or wildcard operator
        default case in switch-case

        _ => do this

        If our variable doesn't match any match arm => do this
    */
    let result = match my_number {
        0 => "Zero",
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ => {
            println!("{} didn't match", my_number);
            "A number between 3 and 255"
        }
    };

    println!("result is {}", result)
}
