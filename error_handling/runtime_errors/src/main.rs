fn main() {
    /*
        Rust has several feature to handle runtime errors.

        Other programming languages handle runtime errors by exceptions but Rust doesn't have exceptions

                    Runtime Errors
                    /          \
                   /            \
                  /              \
            Unrecoverable    Recoverable
                /                 \
               /                   \
              /                     \
     e.g: index out of bounds    e.g: file not found
            /                         \
           /                           \
          /                             \
       panic!()                      Result<T,E>

       panic! macro
        Immediately terminate the program and provide feedback about error

       panic!("Houston, we've had a problem");
    */

    let countdown = [5, 4, 3, 2, 1, 0];

    /*
        Our program will panic when count is 0.

        Dividing a number to 0 will terminate your program
    */
    for count in countdown {
        println!("T-minus {}", count);
        let x = 1 / count;
    }
}
