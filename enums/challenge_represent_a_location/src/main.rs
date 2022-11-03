enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64), // Latitude, longitude
}

trait Display {
    fn display(&self);
}

impl Display for Location {
    fn display(&self) {
        match self {
            Location::Unknown => println!("It's an unknown address"),
            Location::Anonymous => println!("It's an anonymous address"),
            Location::Known(latitude, longitude) => {
                println!("The coordination of address is {}, {}", latitude, longitude)
            }
        }
    }
}

impl Location {
    fn display(&self) {
        match self {
            Location::Unknown => println!("It's an unknown address"),
            Location::Anonymous => println!("It's an anonymous address"),
            Location::Known(latitude, longitude) => {
                println!("The coordination of address is {}, {}", latitude, longitude)
            }
        }
    }
}

fn main() {
    /*
        Define an enum named Location to represent a location

        Three possible values
            Unknown
            Anonymous
            Known - with latitude and longitude stored as float values

        Implement a display() method to print location information.

        - Output should different based on the variant
    */

    let address = Location::Unknown;
    address.display();

    let address = Location::Anonymous;
    address.display();

    let address = Location::Known(28.608295, -80.604177);
    address.display();
}
