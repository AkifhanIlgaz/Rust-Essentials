fn main() {
    let number = Some(13);

    /*
        Here, we check only one possibility.

        if number is Some(13) => print thirteen
        otherwise, do nothing

        "if let" will do the same thing as match but in slighlty simpler syntax ( Syntactic sugar)

        if let <Pattern> = <value> {
            do something...
        }

        match <value> {
            <Pattern> => do something,
            _ => ()
        }
    */
    match number {
        Some(13) => println!("thirteen"),
        _ => (),
    }
    // These are doing the same thing
    if let Some(13) = number {
        println!("Thirteen")
    }
}
