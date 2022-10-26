/*
    Derivable Traits

        Eq                      Clone
        PartialEq               Copy
        Ord                     Hash
        PartialOrd              Default
                                Debug

    immediately before the struct definition

    #[derive(<TraitName1>, <TraitName2>)]
*/
#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };

    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42,
    };

    println!("hubble == gps is {}", hubble == gps);
    println!("hubble > gps is {}", hubble > gps);
}
