use std::any;
use std::fmt;
/*
   Trait Bounds

   In our print_type() function T can be any data type
   In order to print the item , item variable must implement Display trait

   So, T must implement Display trait and we can bound

   <T: std:fmt::Display>

   T can be only data types that implement Display trait
*/
fn print_type_display<T: fmt::Display>(item: T) {
    println!("{} is {}", item, any::type_name::<T>());
}

fn print_type_debug<T: fmt::Debug>(item: T) {
    println!("{:?} is {:?}", item, any::type_name::<T>());
}

fn main() {
    print_type_display(3);
    print_type_display(3.);
    print_type_display("Three");

    print_type_debug([3]);
}
