// Constants use the const keyword.
// A constant must always be annotated with a type.
// Constants can be declared in any scope.
// They can only be assigned constant expressions.
// Rust's naming convention is to use upper case words separated by underscores.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    // This variable shadows the first x.
    let x = x + 1;

    // These curly braces create a new scope.
    {
        // This variable shadows the second x.
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    // Because the scope ends on the previous line, second x is no longer shadowed by third x.
    println!("The value of x is: {x}");
}
