/*
    best_fuel function always returns x

    but anyway, fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str will throw an error
    best_fuel function doesn't use y but we enforce the compiler that x and y must have same lifetime

    If we delete lifetime annotation of y it will be okay.
    But when someone reads our code later , they can be confused about whether we forget to include the lifetime of y or we did it on purpose

    So, annotating different lifetime for y will make our code more robust
*/

fn best_fuel<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        x
    }
}

fn main() {
    let result;

    let propellant1 = "RP-1".to_string();
    {
        let propellant2 = "LNG".to_string();
        result = best_fuel(&propellant1, &propellant2);
    }

    println!("The best fuel is {}", result);
}
