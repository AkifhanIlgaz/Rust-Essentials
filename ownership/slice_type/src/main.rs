fn main() {
    /*
        Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

        A slice is kind of a reference, so it doesn't take ownership
    */

    /*
        String slices is a reference to part of a String
    */

    let message = String::from("Hello World");

    let hello = &message[0..5];
    let world = &message[6..11];

    /*
        We create slices using [starting index, ending index]
            starting index => First position in the slice
            ending index => One more than last position in the slice

        Internally, the slice data structure stores the first position and the length of the slice.
        The length is ending position - starting position

            s
          pointer ===========>  h
          length => 11          e
          capacity => 11        l
                                l
                                o
           world
          pointer ==========>   w
          len => 5              o
                                r
                                l
                                d
    */
    let s = String::from("hello");
    // If you want to start at index zero, you can drop the value before the ".."
    let slice = &s[0..2]; // "he"
    let slice = &s[..2]; // "he"

    // If your slice includes the last bytes, you can drop the trailing value
    let len = s.len();
    let slice = &s[3..len]; // "lo"
    let slice = &s[3..]; // "lo"

    // If you drop both values, you take a slice of entire string
    let slice = &s[0..len]; // "hello"
    let slice = &s[..]; // "hello"
}
