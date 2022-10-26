use std::fmt;

fn compare_and_print<T, U>(a: T, b: U)
where
    T: fmt::Display + PartialEq + From<U>,
    U: fmt::Display + PartialEq + Copy,
{
    if a == T::from(b) {
        println!("{} is equal to {}", a, b)
    } else {
        println!("{} is not equal to {}", a, b)
    }
}

fn main() {
    compare_and_print(1., 1);
    compare_and_print(1.1, 1);
}
