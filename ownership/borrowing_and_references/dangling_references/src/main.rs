fn main() {
    /*
        In languages with pointers, like C/C++, it very easy to create a dangling pointer -a pointer that references to a location in memory that may have been given to someone else- by freeing some memory while preserving the pointer to that memory.

        In Rust, compiler guarantess that if you have reference to some data, the data will not go out of the scope before the reference to the data does
    */

    let reference_to_nothing = dangle();
}

// Returns a reference to a String
fn dangle() -> &String {
    let s = String::from("Hello"); // s goes into scope

    &s // We return the reference to s

    // value of "s" will be dropped when the function ends, so our reference will reference to an invalid String
} // s goes out of scope and is dropped. Its memory is freed.
