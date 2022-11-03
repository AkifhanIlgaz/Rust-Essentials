use std::f32::consts::PI;

/*
    Enum => Enumeration

    Enums define a data type that can be one of possible variants.

    Our Color enum can be Red,Green or Blue.
*/
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum Shape {
    Circle(f32), // radius
    Rectangle(f32, f32),
    Triangle(f32, f32, f32),
}

fn main() {
    let color = Color::Blue;

    let my_shape = Shape::Triangle(3., 2., 5.);

    let shape_perimeter = calc_perimeter(&my_shape);

    println!("my_shape: {:?}\nperimeter: {}", my_shape, shape_perimeter)
}

fn calc_perimeter(shape: &Shape) -> f32 {
    /*
        Match is very similar to switch-case statements

        We have a value and we will try to match this value with one of the arms

        In our case, we are trying to match shape variable

        shape is a Shape enum which means that it can be Circle, Rectangle or Triangle

        If shape is Shape::Circle => do this
        If shape is Shape::Rectangle => do this
        If shape is Shape:Triangle => do this

        match operator must be exhaustive which means that match arms must cover all possible values of enum
    */
    match shape {
        Shape::Circle(r) => (2. * PI * r),
        Shape::Rectangle(width, height) => 2. * (width + height),
        Shape::Triangle(x, y, z) => x + y + z,
    }
}
