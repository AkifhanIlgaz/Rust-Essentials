fn main() {
    /*
        We know that references immutable by default, but we can make them mutable just like we did to variables
    */

    let mut s1 = String::from("Hello"); // We will modify the value, so s1 should be immutable

    change(&mut s1);

    /*
        Mutable references is useful but they have one some restriction:
            If you have a mutable reference to a value, you cannot have another references to that value

        This is different that other programming languages. Rust prevents data races with this restriction.

        Data race happens when these three behaviours occur:
            Two or more pointer are trying to acces the same data at the same time
            At least one of the pointers is being used to write to the data
            There's no mechanism being used to synchronize access to the data
    */

    let mut string = String::from("Hello");

    let mut s1 = &mut string;
    let mut s2 = &mut string;

    println!("{}, {}", s1, s2);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
    /*
       The user of r3 can modify the value of s. But the users of r1 and r2 doesn't expect the value to suddenly changes.

       Note that, multiple immutable references are allowed since none of the is allowed to modify the value
    */
}

fn change(s: &mut String) {
    // Creates a mutable reference to "s"
    // Now we can modify,use the "s" without taking ownership of it.
    s.push_str(" World");
}
