use std::fs;
fn main() {
    /*
        fs module of std library enables us to read from files.
    */

    // Converts the file in the specified path into the string
    let planets = fs::read_to_string("planets.txt").unwrap();

    println!("The planets are {}", planets);

    //  We can also read the file in the context of lines with lines() method

    for (index, planet) in planets.lines().enumerate() {
        println!("The {}th line is {}", index + 1, planet);
    }

    /*
        Since our planets file is a text file which means it is a string, it's easy to work with.

        What if we have to read from non-string files, such as images or videos? We have to work with the bytes

        We will use fs::read(path) function to do that.
        read(path) function will return Vec<u8>, a.k.a byte-array
    */

    let planets = fs::read("planets.txt").unwrap();

    println!("Planets are {:?}", planets);
}
