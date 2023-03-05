fn main() {
    let config_max = Some(3u8);
    // The if let control flow allows us to do something, if a variables matches a pattern.
    // This is roughly the same as using if (x is y) in C#.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    // Rust also allows an else case to be used.
    // It's also possible to use else if let.
    // This is interesting, because Rust does not allow else blocks without curly braces.
    } else {
        println!("This is printed in the else case!");
    }
}