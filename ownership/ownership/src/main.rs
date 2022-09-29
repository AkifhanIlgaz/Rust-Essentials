fn main() {
    /*
        Ownership is a set of rules that governs how a Rust program manages memory.

        Programmer doesn't need to explicitly allocate and deallocate memory, ownership rules will handle it. Because of that Rust allows to write more memory-safe code

        There is no garbage collector that slows down our program.

        Rust compiler applies the rules of ownership to our code before runtime and if there is an error, it will not compile.

        This is what makes Rust is a memory-safe,efficient great language
    */

    /*
        Ownership Rules

        1) Each value in Rust has an owner
        2) There can only be one owner at a time
        3) When the owner goes out of scope, the value will be dropped

        The scope concept is very similar, even the same, to that in other programming languages. Because of that, we will not explain what is scope
    */

    // message is not valid here.
    {
        let message = "Hello World"; // message is valid from this point forward
        println!("The message is: {message}"); // OK
    }
    // The scope is over now.

    println!("The message is: {message}"); // cannot find value "message" in this scope

    /*
        "message" is the owner of "Hello World" value
        "message" goes out of scope which means that it is no longer valid,accessible
        Since "message" goes out of scope "Hello World" value is dropped, deleted from the heap
    */
}
