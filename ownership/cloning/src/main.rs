fn main() {
    /*
        If we want to deeply copy the heap data of String, we can use common method "clone"

        "clone" method creates a fresh-new, same value on the Heap and binds it to new variable. Therefore, the ownership of the value isn't moved.

    */

    let world = String::from("Earth"); // Owner of "Earth" is "world"
    let blue_marble = world.clone(); // Owner of the newly created "Earth" is "blue_marble"
}
