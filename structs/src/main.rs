struct Color(u8, u8, u8); // RGB
#[derive(Debug)]
struct Point(u8, u8, u8); // X Y Z Coordinates

#[derive(Debug, Clone)]
struct Shuttle {
    name: String,    // Field
    crew_size: u8,   // Field
    propellant: f64, // Field
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.,
        }
    }
}
fn main() {
    let mut vehicle = Shuttle::new("Endeavour");
    let vehicle2 = Shuttle::new("Atlantis");
    println!("Propellant: {}", vehicle.propellant);
    vehicle.add_fuel(1.);
    println!("Propellant: {}", vehicle.propellant);

    let red = Color(255, 0, 0);
    println!("First value is: {}", red.0);

    let origin = Point(0, 0, 0);
    let y_value = get_y(&origin);
    println!("{}", y_value);
    println!("{:?}", origin);
}

fn get_y(p: &Point) -> u8 {
    p.1
}
