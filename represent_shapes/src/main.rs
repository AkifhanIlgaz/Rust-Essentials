/*
    Create a Rectangle struct with these features:
        Fields
            Width
            Height
        Methods
            get_area() -> Area of the rectangle
            scale(scalar) -> Scale the rectangle with the given "scalar"
        Associated Functions
            new(width,height) -> Creates a Rectangle instance with given parameters and return it
*/

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, scalar: f64) {
        self.height *= scalar;
        self.width *= scalar;
    }

    fn new(_width: f64, _height: f64) -> Rectangle {
        Rectangle {
            width: _width,
            height: _height,
        }
    }
}

fn main() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed")
}
