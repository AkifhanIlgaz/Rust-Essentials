fn main() {
    /*
        Rust compiler uses borrow checker to determine whether all borrows are valid

        Lifetime for propellant
            'a => from line 16 to line 29

        Lifetime for rp1
            'b => from line 18 to line 22

        For print statement on line 19 everything is okay. Both variables are valid

        But on print statement on line 28 we will encounter an error. rp1 is no longer available
    */

    let propellant;

    {
        let rp1 = "RP-1".to_string();
        propellant = &rp1;
        println!("The propellant is {}", propellant);
    }

    // Code below will throw an error. ^^^^ borrowed value does not live long enough
    // propellant is a reference to "rp1" variable
    // rp1 goes out of scope at line 12 and it is no longer accessible
    // propellant points to an invalid address which is called dangling reference
    println!("The propellant is {}", propellant);
}
