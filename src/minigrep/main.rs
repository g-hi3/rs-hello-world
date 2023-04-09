use minigrep::Config;
use std::{env, process};
use std::error::Error;

fn main() {
    // This is basically the equivalent of `string[] args` in C# and `String[] args` in Java.
    // The Rust book mentions `args_os()` and the `OsString` type here, but doesn't explain in detail.
    // From what I understand, `args()` returns an iterator, which is comparable to IEnumerable in C#.
    // The `collect()` function acts similar to `ToList()` in C# and evaluates the iterator.
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)
        // The Rust book calls the argument a closure (or anonymous function).
        // At this point, I'm not sure if I could pass a function.
        .unwrap_or_else(|err| {
            eprintln!("Unable to parse config: {err}");
            process::exit(1);
        });

    println!("Searching for {} in file {}", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}