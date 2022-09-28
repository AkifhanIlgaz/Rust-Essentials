fn main() {
    /*
        Integers are whole number without fractional component, such as, 1, 250, 77, 65 etc.

        There are two types of integers:

        Unsigned Integer (uint) => Only Positive
        Signed Integer (int) => Negative or Positive

        Rust automatically checks for overflows/underflows
    */

    // We specify the data type of an integer by number of bits used to represent it.For example:

    /*
        00000001 => 8 bit

        uint variants => u8, u16, u32, u64, u128

        uN type integer can store [0, 2^(N) - 1]
    */

    let x: u8 = 1; // [0, 2(8) - 1] =  [0,255]
    let x = 1u8;
    /*
        0000000000000001 => 16 bit

        int variants => i8, i16, i32, i64, i128

        iN type integer can store [-(2^(N-1)), 2^(N-1) - 1]
    */

    let y: i16 = 1; // [ -2(15), 2(15) -1]

    /*
        There is one more integer type whose size can change depending on the architecture of the computer
        isize , usize
        They can be use for indexing purposes on arrays etc.
    */

    /*
        Integer Literals

        Decimal  =======> 98_325  You can use "_" to make easy reading the number
        Hex =====> 0xff
        Octal ====> 0o77
        Binary ====> 0b1111_0000
        Byte(u8 only) ====> b'A'
    */

    // If you don't specify the data type of the number Rust stores integers as i32 by default
    let default_number = 25; // i32
}
