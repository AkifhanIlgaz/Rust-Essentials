use std::f32::consts::PI;

#[derive(Debug)]
enum Shape {
    Circle(f32),
    Rectangle(f32, f32),
    Triangle(f32, f32, f32),
}

impl Shape {
    fn get_perimeter(&self) -> f32 {
        match *self {
            Self::Circle(r) => 2. * PI * r,
            Self::Rectangle(w, h) => 2. * (w + h),
            Self::Triangle(a, b, c) => a + b + c,
        }
    }
}

fn main() {
    let my_shape = Shape::Rectangle(2., 3.);
    println!("my_shape is {:?}", my_shape);

    let perimeter = my_shape.get_perimeter();

    println!("Perimeter is {}", perimeter);
}
