// In contrast to vectors, hash maps need a use statement.
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores
        // Just like for vector, get returns an Option<&i32>
        .get(&team_name)
        // copied converts the Option<&i32> to Option<i32>.
        .copied()
        // unwrap_or is comparable to C#'s GetValueOrDefault.
        .unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Values that implement the Copy trait will be copied into the hash map.
    // For owned values like String, values will be moved.
    // References to values won't be moved into the hash map.
    // Values in the hash map must be valid for at least as long as the hash map is valid.

    // insert overrides the value.
    scores.insert(String::from("Blue"), 20);
    scores
        // entry returns an enum of whethere there is a value for this key.
        .entry(String::from("Blue"))
        // or_insert only inserts when there is no value with this key.
        .or_insert(89);

    let text = "Hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map
            .entry(word)
            // or_insert returns a reference to the value at that position or the value inserted.
            .or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    // Hash maps in Rust use SpiHash.
    // Other hashers can be used with types that implement BuildHasher.
}