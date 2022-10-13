// We need to implement PartialOrd trait to T.
// In order to compare to values we need to know how we can compare them
// We are looking bigger number, so it is a binary comparison
// T must be a numeric data type
fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("biggest is {}", get_biggest(3, 2));
}
