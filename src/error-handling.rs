// Rust knows two kinds of errors:
// * Recoverable, by using the Result<V, E> enum
// * Unrecoverable, by using the panic! macro
// There are no exceptions in Rust.

// When a program panics, Rust will:
// * print a failure message
// * unwind
// * clean up the stack
// * quit
// Via an environment variable, Rust can be made to print the call stack when a panic occurs.
// RUST_BACKTRACE=1
// Unwinding means that Rust walks back up the stack and cleans up the data from each function.
// Because this is a lot of work, Rust allows an alternative of immediately aborting.
// Add [profile.release]\npanic = 'abort' to the toml file.

fn main() {
    // This causes the program to panic.
    panic!("crash and burn");
}