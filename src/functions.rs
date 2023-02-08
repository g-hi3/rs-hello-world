// Rust convention for functions and variables is to use snake case.
// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value. Let’s look at some examples.

fn main() {
    // This block evaluates to 4.
    // This means, the value of y is 4.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // Rust doesn't care where you define your functions.
    // It only cares that it's defined in a scope that the caller can see.
    print_labeled_measurement(5, 'h');

    println!("five is {}", five())
}

// Parameters must be annotated with a type.
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// The return type of a function must be annotated with ->.
// Rust does not require the `return` keyword for the statement at the end of a function.
// To return early it is required, however.
// Adding a semicolon to the end of the last expression makes it a statement and therefore causes an error.
fn five() -> i32 {
    5
}