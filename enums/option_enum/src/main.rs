fn main() {
    /*
        How do you represent nothing ?

        In many languages, "null" value is used to represent nothing. It means that there is no value

        But this causes runtime error, when using a null value in a not-null context.

        Let's say we want to find the last node in a linked list.
            1 => 2 => 3 => 4 => null

            JavaScript

            let current = head;
            while(current) {
                current = current.next
            }

            console.log(current.val)

        When we are at the node(4) next pointer will point "null" value which means that we are at the end of the linked list.

        Now our current value is "null" but in console.log() we want to read the value of current.

        current is "null", it will throw a runtime error.Because we are trying to access a value that doesn't exist.

        Rust doesn't have traditional "null" value.

        Instead of that, Rust uses Option<T> enum.

        enum Option<T> {
            Some(T),
            None
        }

        Option enum is included in prelude, so you don't need to import anything to use it
    */

    let something = Some(13.); // Option<f64>
    let something = Some(13); // Option<i32>
    let something = Some("13"); // Option<&str>

    let nothing: Option<()> = None;

    let countdown = [5, 4, 3, 2, 1];

    // let number = countdown[5]; // Index out of bounds

    /*
        countdown.get() will return an Option

        If index is valid returns Some(countdown[index]);
        If index isn't valid returns None
    */

    let number = countdown.get(5);
    let number = number.unwrap_or(&0) + 1;

    println!("number is {:?}", number);
}
