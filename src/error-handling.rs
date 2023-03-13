// Rust knows two kinds of errors:
// * Recoverable, by using the Result<T, E> enum
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
// Add this to the toml file:
// [profile.release]
// panic = 'abort'

use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;

// The main function can also return a Result<(), E>, to support the ? operator.
// This means the return expression should be an appropriate error or Ok(()).
// The Box<dyn Error> is a trait object.
// It means it could be any kind of error.
// The main function may return any type that implements the std::process::Termination trait.
fn main() -> Result<(), Box<dyn Error>> {
    // The success value is a std::fs::File.
    // The error value is std::io::Error.
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };

    // Alternatively, we can use unwrap_or_else.
    let another_file = File::create("hello.txt")
        // This is interesting syntax, because the lambda can be used with optional curly braces, when there is a single statement.
        .unwrap_or_else(|error| panic!("Problem creating the file: {:?}", error));

    let unwrap_file = File::open("hello.txt")
        // Unwrap returns the value, or call the panic! macro when there is an error.
        .unwrap();

    let expect_file = File::open("hello.txt")
        // expect calls the panic! macro when there is an error and pass the string to it.
        .expect("hello.txt should be included in this project");

    // In such a situation, the developer has more information than the compiler.
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

    // It is advisable to panic, when the program is in an invalid state or has contradictory values.
    // When a user passes bad data, it is more useful to return an error with information about how the data is wrong.
    // Expected failure should also be wrapped inside a Result, such as I/O errors.
    // When a contract is broken, the code needs to panic, because the error is caused by the calling code.

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    // The ? operator at the end of the statement can be used to propagate errors.
    // This operator can only be used when the return type is Result with the appropriate error type.
    // It can also be used with Option enums, when the return type is also an Option.
    // The book mentions something about the from function in the From trait, but I don't understand it well enough yet.
    File::open("hello.txt")?
        .read_to_string(&mut username)?;
    Ok(username)

    // Actually, this whole function could be replaced by fs::read_to_string("hello.txt").
}

pub struct Guess {
    // Because this value is private, only functions in the impl block may change it.
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> {
        // This guard clause makes sure the input value may only be between 1 and 100.
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    // This impl block only has a getter, so the value can never change.
    // And because the input is only ever 1 to 100, it can never be outside of that range.
    pub fn value(&self) -> i32 {
        self.value
    }
}