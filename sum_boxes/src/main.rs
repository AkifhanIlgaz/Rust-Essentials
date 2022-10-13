fn sum_boxes<T: std::ops::Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T> {
    Box::new(*a + *b)
}

fn is_equal<T: std::cmp::PartialEq>(a: &Box<T>, b: &Box<T>) -> bool {
    *a == *b
}

fn main() {
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one, two), 3);

    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(pi, e), 5.85987);

    let first = Box::new(0);
    let second = Box::new(0);
    let third = Box::new(3);
    assert_eq!(is_equal(&first, &second), true);
    assert_eq!(is_equal(&first, &third), false);
    println!("Tests are passed");
}
