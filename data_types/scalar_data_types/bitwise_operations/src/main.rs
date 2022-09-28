fn main() {
    let mut value = 0b1111_0101u8; // Stored as u8

    // Since Rust stores it as an integer, print macro will display is as a normal integer
    println!("The value is :{}", value); // 245

    // How can we display it in binary ?
    // {:[Leading number][How many bits to display][which format]}   b => binary x => hexadecimal o => octal
    // {:08b}  8 bits in binary with leading zeros if possible
    println!("The value in binary is: {:08b}", value); // 11110101
    println!("The value in binary is: {:09b}", value); // 011110101
    println!("The value in binary is: {:04x}", value); // 00f5 Hexadecimal
    println!("The value in binary is: {:06o}", value); // 000365 Octal

    /*
        Bitwise operations
    */

    // NOT => !
    value = !value;
    println!("Value is {:08b}", value); // 00001010

    // AND => &

    // We can use AND operator if we want to clear a specific bit
    // Let's say we want clear 4th bit from left
    // value & aux
    // aux is an integer whose all bits are set to 1 except the one that we want to clear
    let aux = 0b1111_0111;
    value = value & aux;
    println!("Value is {:08b}", value); // 00000010

    // We can also use AND for checking value of a specific bit
    // We want to check if the 6th bit
    // value & aux
    // aux is an integer whose all bits are set to 0 except the one that we want to check;
    println!("6nd bit is {}", value & 0b0000_0010); // 0

    // OR => |
    value = value | 0b0100_0000;
    println!("Value is {:08b}", value); // 01000010

    // XOR => ^
    // There is a nice trick.
    // a ^ b = c      c ^ a = b
    value = value ^ 0b0101_0101;
    println!("Value is {:08b}", value); // 00010111

    // LEFT SHIFT => <<    RIGHT SHIFT => >>
    value = value << 4;
    println!("Value is {:08b}", value); // 01110000
    value = value >> 4;
    println!("Value is {:08b}", value); // 00000111
}
