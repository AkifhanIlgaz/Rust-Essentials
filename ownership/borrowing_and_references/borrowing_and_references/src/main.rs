fn main() {
    /*
        A reference is like a pointer in that it's an address we can follow to access the data stored at that address; and that data is owner by another variable

        Unlike pointers in C/C++, a reference is always points to a valid value of a particular type for the life of that reference

    */

    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of {s1} is: {len}");

    /*
          s             s1              HEAP

       pointer  ====> pointer ========> h
                        len             e
                      capacity          l
                                        l
                                        o

    */

    // We call the action of creating a "reference borrowing". As in real life, if a person owns something, you can borrow it from them. When you're done, you have to give it back. You don't own it

    let s2 = String::from("hello");

    change(&s2); // Will throw an error. We borrowed something from our friend and now we are trying to change it, but we don't have the right to do that since we are not the owner of it

    // As variables, references are immutable by default. We cannot modify something we have reference to
}

fn change(s: &String) {
    s.push_str("World"); // This line will throw an error
}

fn calculate_length(s: &String) -> usize {
    // Instead of taking ownership of the parameter it creates a reference to the owner of it.
    // In order to access the value first we go to the owner of that value
    s.len()
}
