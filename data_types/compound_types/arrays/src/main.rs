fn main() {
    /*
        Arrays are another way to store a collection of multiple values, but there are a few restrictions about it:

        All elements in the array must have the same data type
        Arrays are fixed-size

        Since we know the size of the array at compile time we can allocate it on the stack
    */

    let arr = [1, 2, 3];

    /*
        Array's type
        [<data_type_of_elements>; <number_of_elements>]
        Let's write the type of the array that we have just created
    */

    let arr: [i32; 3] = [1, 2, 3];

    // You can initialize an array that contains the same value for each element
    // [<value>;<length>]

    let init_array = [0; 5]; // [0,0,0,0,0]

    // You can use len() method for the length of an array;
    let length = init_array.len();
    println!("The length of the init_array: {}", length);

    /*
        Multi-dimensional Array
    */

    let two_D_array = [[1, 2], [3, 4]];

    // Inner dimensions must have same length and same data type

    let wrong_2_d_array = [[1, 2], [3, 4, 5]]; // Error
}
