struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32, // miles
}

// Rust has some useful built-in traits, also you implement your own trait
/*
    We override describe() function for SpaceStation, SpaceStation.describe() will call SpaceStation's describe()

    We don't write a specific describe() function for Satellite struct, so Satellite.describe() will call default describe() function
*/
trait Description {
    // Default trait implementation
    fn describe(&self) -> String {
        String::from("An object flying through space")
    }
}

impl Description for Satellite {}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!(
            "The {} flying {} miles high with {} crew members aboard!",
            self.name, self.altitude, self.crew_size
        )
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };

    let iss = SpaceStation {
        name: String::from("Internation Space Station"),
        crew_size: 6,
        altitude: 254,
    };

    println!("Hubble is {}", hubble.describe());
    println!("ISS is {}", iss.describe());
}
