fn main() {
    let s = String::from("hello world");
    // word will get the value 5.
    let word = first_word(&s);

    // word still has the value 5 here.

    // The expression within the brackets is called a range.
    // The slice will be created within this range (mathematical: [0..5[).
    // Because 0 is the starting index, it can be dropped from the range expression.
    let hello = &s[..5];
    // The ending index may also be dropped, if the string slice covers the end of the string.
    let world = &s[6..];

    // Clippy says, variables can be used directly in the format string.
    println!("word: {word}");
    println!("hello: {hello}");
    println!("world: {world}");

    let s2 = first_word("hello world");
    println!("s2: {s2}");

    // String slice range indices must occur at valid UTF-8 character boundaries.
    // If a string slice ends in the middle of a multibyte character, the program will panic.

    // Slices also work with arrays:
    let a = [1, 2, 3, 4, 5];
    // &[i32] is the type for an int array slice.
    let _slice: &[i32] = &a[1..3];
}

// Using the parameter type &str allows us to call it using &str or &String.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}