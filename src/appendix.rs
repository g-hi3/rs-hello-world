fn main() {
    let a = "hello world";
    let b = "world";
    // When using a name with raw prefix, the raw prefix must also be applied.
    println!("a matches b: {}", r#match(a, b));
}

// Using the `r#` prefix (or raw identifier), a name can be the same as a reserved keyword.
fn r#match(a: &str, b: &str) -> bool {
    a.contains(b)
}