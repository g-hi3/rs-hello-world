//! # My Crate
//!
//! This is a crate description.
//! According to the Rust book, it can be used to document containing elements.
//! It can be a crate root file or a module.

fn main() {

}

/// This is a documentation comment.
/// It generates an HTML document.
///
/// # Examples
///
/// A code example like this will also be run by `cargo test` as a doc test.
///
/// ```
/// let arg = 5;
/// let anwer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///
/// To generate documentation, run `cargo doc` or `cargo doc --open` to open the doc immediately.
/// Commonly used sections are Examples, Panics, Errors, Safety.
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// To login to crates.io, use the `cargo login <API key>` command.
// This creates a secret token at ~/.cargo/credentials.