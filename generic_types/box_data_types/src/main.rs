use std::mem;
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    /*
        Box<T> Data Types

        Stores data on the heap instead of on the stack even if it has a fixed known size

            Stack                           Heap
          ptr => 200 ====================>   T   200

        Boxed are considered smart pointers because they have some additional functionality

        Smart Pointers

            Provide additional functionality beyond references
            Box<T> has ownership of the data it points to
            When Box<T> goes out of scope it deallocates the heap memory

    */

    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 4984161.0,
    };

    println!(
        "Vehicle size on the stack: {} bytes",
        mem::size_of_val(&vehicle)
    ); // 40 bytes

    // This box is pointing to a shuttle
    // boxed_vehicle takes the ownership of vehicle value
    // vehicle data is moved to the heap
    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);

    // Now boxed_vehicle only stores the pointer to the vehicle value on the heap
    println!(
        "boxed_vehicle size on the stack: {} bytes",
        mem::size_of_val(&boxed_vehicle)
    ); // 8 bytes

    // mem::size_of_val(&boxed_vehicle) only returns the size of the our pointer that points to the vehicle on the heap
    // What if we want to know the size of the value that our boxed_vehicle points to ?

    /*
        Dereference Operator
            Represented with "*" symbol
            When it is applied to a pointer it denotes the pointed-to location

            *boxed_vehicle => Returns the data that boxed_vehicle points to
    */
    println!(
        "boxed_vehicle size on the heap: {} bytes",
        mem::size_of_val(&*boxed_vehicle)
    );

    // Let's say we want to take a data from the heap and store it on the stack
    // Takes the ownership of boxed_vehicle and now it is stored on the stack
    let unboxed_vehicle: Shuttle = *boxed_vehicle;
    println!(
        "unboxed_vehicle size on the stack: {} bytes",
        mem::size_of_val(&unboxed_vehicle)
    );
}
