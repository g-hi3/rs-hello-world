// ## Integer Types
// | Length | Signed | Unsigned | Literal |
// | --- | --- | --- | --- |
// | 8 bits | `i8` | `u8` | `69i8` or `69u8` |
// | 16 bits | `i16` | `u16` | `69i16` or `69u16` |
// | 32 bits | `i32` (default) | `u32` | `69i32` or `69u32` |
// | 64 bits | `i64` | `u64` | `69i64` or `69u64` |
// | 128 bits | `i128` | `u128` | `69i128` or `69u128` |
// | arch (32 or 64 bits) | `isize` | `usize` | `69isize` or `69usize` |
//
// ### Integer Literals in Rust
// | Number literals | Example |
// | --- | --- |
// | Decimal | 98_22_i32 |
// | Hex | 0xff_i32 |
// | Octal | 0o77_i32 |
// | Binary | 0b1111_0000_i32 |
// | Byte (`u8` only) | b'A' |
//
// ## Floating-Point Types
// | Length | Type | Literal |
// | --- | --- | --- |
// | 32 bits | `f32` | 5.0f32 |
// | 64 bits | `f64` (default) | 5.0f64 |
//
// ## Boolean Type
// The boolean typ in Rust is called `bool`.
// The literals for it are `true` and `false`.
//
// ## The Character Type
// The character type in Rust is called `char`.
// Its literal uses single quotes as opposed to the string literal which uses double quotes.
// The size of `char` is four bytes and represents a Unicode scalar value.

// Overflows in debug mode cause a panic (program exit with error).
// Overflows in release mode cause the value to wrap around (256u8 would be 0).
fn main() {
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1
    let _remainder = 43 % 5;
    let _t = true;
    let _f: bool = false;
    let _c = 'z';
    let _z: char = 'ℤ';
    let _heart_eyed_cat = '😻';
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Rust also supports destructuring.
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");
    // Tuples can also be accessed with an index.
    // However, it can not be passed in a formatting string.
    println!("The value of x is: {}", tup.0);
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _b = [3; 5]; // [3, 3, 3, 3, 3]
    // Invalid array access causes the program to panic.
    let _element = _b[0];
}