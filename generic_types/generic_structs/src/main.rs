#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

#[derive(Debug)]
struct Rectangle1<T> {
    width: T,
    height: T,
}

fn main() {
    /*
        Generic Data Types
            Abstract stand-ins for concrete data types or other properties

        Let's say we want to instantiate our Rectangle with u8 values.

        One solution is that creating separate structs for each data types.Such as,

        struct Rectangle_f64 {
            width: f64,
            height: f64,
        }
        struct Rectangle_u8 {
            width: u8,
            height: u8,
        }

        But creating a struct for each data type is a little bit tedious.

        We can use generic data types for that purpose.

        struct Rectangle<T> {
            width: T,
            height: T,
        }
    */
    let rect1 = Rectangle1 {
        width: u8,
        height: u8,
    };
    // Since we use u8 for our with field every field that has T for data type must have u8 data type

    // If we have different generic types we can add more variables
    let rect = Rectangle {
        width: 3u8,
        height: 2u16,
    };

    println!("rect is {:?}", rect);
}
