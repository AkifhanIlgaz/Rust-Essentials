#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width // We return a reference since we are using a generic data type
                    // We don't know the data type of the width field.
                    // If it is heap-based data type the ownership of the width will be moved
                    // To prevent this, we are returning a reference to it
    }
}

// These methods will only apply to Rectangle instances that has specified data types
// In our example, get_perimeter() function is only available for Rectangle<u8, u8> instances
impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect = Rectangle {
        width: 3u8,
        height: 2u8,
    };

    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
    println!("perimeter is {}", rect.get_perimeter());
}
