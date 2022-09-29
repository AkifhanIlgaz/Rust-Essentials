fn main() {
    /*
        For loops are used to iterate through a collection
    */

    let collection = [1, 2, 3, 4, 5];

    // Iterates through all elements in the collection array
    for element in collection {
        println!("The element is :{element}");
    }

    /*
      for loops uses ranges very often
      Range => How many times we have to iterate through

      .. => Exclusive range => 0..3 => [0, 3)
      ..= => Inclusive ranger => 0..=3 => [0, 3]
    */

    for number in 0..3 {
        print!("The number is {number}") // 012
    }

    for number in 0..=3 {
        print!("The number is {number}") // 0123
    }

    // If you don't use the varible that is declared in for loop use "_" or prefix it with "_"

    for _ in 0..3 {
        print!("Printing the same thing three times")
    }

    for _number in 0..3 {
        print!("Printing the same thing three times")
    }

    // rev() method reverses the order of range

    for number in (1..=3).rev() {
        println!("{number}");
        // 3
        // 2
        // 1
    }

    println!("LIFTOFF!!");
}
