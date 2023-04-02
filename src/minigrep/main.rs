use std::{env, fs};

fn main() {
    // This is basically the equivalent of `string[] args` in C# and `String[] args` in Java.
    // The Rust book mentions `args_os()` and the `OsString` type here, but doesn't explain in detail.
    // From what I understand, `args()` returns an iterator, which is comparable to IEnumerable in C#.
    // The `collect()` function acts similar to `ToList()` in C# and evaluates the iterator.
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query} in file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}