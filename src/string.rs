fn main() {
    // Both String and str are UTF-8 encoded.
    // String is a wrapper around a vector of bytes.

    // This creates a new (empty) string.
    let mut s = String::new();

    // A string literal can also be converted to a string by using the to_string method.
    let s = "hello".to_string();

    // These are all valid strings.
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Strings can be concatenated using the + operator or using the format macro.
    let s = "hello" + format(", {}", 15);
    // The + operator uses the add method.

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // Rust uses dereference coercion to convert a &String to &str[].
    // The add method takes ownership of s1, but not s2.

    // Indexing with strings is not allowed.
    // The reason for this is because strings are encoded in UTF-8 and some characters use more than a single byte of memory.
    // Using string slices is allowed, but you have to be careful how to use them.
    // Getting a string slice with the range operator returns a slice with those bytes.
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // s contains Зд at this point.
    // If we were to use &hello[0..1], the program would panic.

    // For individual Unicode scalar values, we can use chars to iterate.
    for c in "Зд".chars() {
        // This will print characters.
        println!("{c}");
    }

    // To iterate over bytes we can use the bytes method.
    for b in "Зд".bytes() {
        // This will print numbers.
        println!("{b}");
    }
}