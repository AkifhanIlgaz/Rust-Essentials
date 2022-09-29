fn main() {
    /*
        Moving
    */

    // Let's look what is happening under the hood when we create a String type variable
    let planet = String::from("Earth");

    /*
        The value "Earth" is stored on the heap and let's say its address is 200.

              HEAP
        200     E
        204     a
                r
                t
                h

        "planet" variable is stored on the heap and made up of three parts:
            pointer
            length
            capacity

            STACK

            planet
         pointer => 200
         length => 5
         capacity => 5

        planet is the owner of "Earth"

    */

    println!("The planet is: {planet}");

    /*
        Let's see what is going on under the hood

        Go to the stack
        Find the planet variable
        Read the pointer field which is 200
        Go to address 200
        Read ( length * 4 ) bytes starting from that address and return it

        We read 4 byte for each letter sinc it is a character and characters are 4 bytes
    */

    // Let's try to copy the value of "planet" to another variable and let's see what's happening

    let blue_marble = planet;

    /*
        The fields of planet is copied into blue_marble and pushed onto the stack

                           STACK

            planet                   blue_marble
         pointer => 200            pointer => 200
         length => 5               length => 5
         capacity => 5             capacity => 5

        Both of the are pointing to the same value

        But according to the ownership rules, a value can have only one owner at a time

        So, our planet variable is deleted and no longer valid

        blue_marble is the owner of the "Earth" now.

        This is called moving in Rust.

        We have moved the ownership of "Earth" from "planet" to "blue_marble"

        In other languages like Javascript, when you do that shallow copy of value is created
    */

    println!("The planet is {}", planet); // !! planet is no longer valid
    println!("The planet is {}", blue_marble); // OK
}
