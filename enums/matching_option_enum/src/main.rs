fn main() {
    let countdown = [5, 4, 3, 2, 1];

    let number = at(&countdown, 0);

    println!("Number is {number}");
}


fn at(arr: &[i32], index: usize) -> i32 {
    match arr.get(index) {
        Some(number) => *number,
        None => 0,
    }
}
