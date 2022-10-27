/*
    Lifetime Annotation

    Explicitly defines a generic lifetime for parameters

    Must begin with apostrophe(') symbol

    Names are conventionally single lowercase letters   'a, 'b etc.

    Borrow checker will use the lifetime of parameter that is more restrictive
*/

fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let result;

    let propellant1 = "RP-1".to_string();

    {
        let propellant2 = "LNG".to_string();
        result = best_fuel(&propellant1, &propellant2);
    }
    println!("The result is {}", result);
}
