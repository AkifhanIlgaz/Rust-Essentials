/*
    Lifetime Elision Rules

    Set of rules for the compiler to analyze reference lifetimes.

    Describes situations that do not require explicit lifetime annotations.

    If any ambiquity remains, explicit annotation will be required.

    ** Lifetime rules are applied to references

    There are currently three rules. If after working through these rules, the compiler is still not able to determine output lifetime, it will require you to include explicit annotations

        1) Each input parameter that is a reference is assigned its own lifetime.

        * fn get_first_word<'a>(x: &'a str, y: i32) -> &str  || No need lifetime annotation for y
        * fn get_longest<'a, 'b>(x: &'a str, y: &'b str) -> &str
        *
        *

        2) If there is exactly one input lifetime, assign it to all output lifetimes

        * fn get_first_word<'a>(x: &'a str) -> &'a str

        Rule 2 doesn't apply for get_longest function. There are multiple input lifetimes
        * fn get_longest<'a, 'b>(x: &'a str, y: &'b str) -> &str

        3) If there is a &self or &mut self input parameter, its lifetime will be assigned to all output lifetimes

        * fn send_transmission(&self, msg: &str) -> &str

            Rule 1
            fn send_transmission<'a, 'b>( &'a self, msg: &'b str) -> &str

            Rule 3
            fn send_transmission<'a, 'b( &'a self, msg: &'b str) -> &'a str

*/

fn main() {
    let message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message);

    println!("first_word is {}", first_word);
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // Found a space
        }
    }

    &s // No space found
}
