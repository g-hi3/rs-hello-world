// The set of use-statements is called the prelude.
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // The range argument syntax is start..=end.
    // It returns a number between 1 and 100 (inclusive upper and lower bound).
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // The loop keyword creates an infinite loop.
    loop {
        println!("Please input your guess:");

        // Variables are immutable by default.
        // By adding the mut keyword, a variable is mutable.
        // Calling functions with the :: operator is like calling a static method in OOP languages.
        // The String::new() expression creates a new, empty string.
        let mut guess = String::new();

        // This line could also have been written as:
        //     std::io::stdin()
        // std::io::Stdin is a type comparable to InputStream in Java.
        io::stdin()
        // The read_line function reads input from the command line and *appends* it to the string parameter.
        // The & indicates that an argument is passed by reference.
        // References are also immutable by default.
        // Because of this, guess is passed with &mut guess as a mutable reference.
        // To pass an immutable reference, use &guess instead.
            .read_line(&mut guess)
        // read_line returns a Result value, which is an enum.
        // Each possible state of an enum is called a variant.
        // Result's variants are Ok and Err.
        // An instance of Result has an expect method.
        // If the variant is Err, expect crashes the program.
        // If the variant is Ok, expect returns the value associated with the Result.
        // Not using expect is allowed, but the compiler issues a warning.
            .expect("Failed to read line!");

        // Using : a variable can be assigned a type.
        // The type u32 represents an unsigned 32 bit number.
        // Rust allows *shadowing* a variable by defining a new variable with the same name as an existing one.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // The underscore here is comparable to the discard _ in C#.
            // The continue keyword will skip to the next iteration of the loop.
            Err(_) => continue
        };

        // String interpolation in rust works without any extra semantics.
        // You can use it like:
        //     println!("string {variable}");
        //     println!("string {}", variable);
        println!("You guessed: {guess}");

        // The match keyword works a lot like switch expressions in C#.
        // The match expression is made up of arms.
        // An arm consists of a pattern and the code that should be run.
        // Because guess is a u32 variable at this point, Rust will infer that secret_number should also be a u32 variable.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            // Assumption: Code in a block creates a new scope.
            Ordering::Equal => {
                println!("You win!");
                // The break keyword will make the code break out of the loop.
                break;
            // The last line of a match statement can have a trailing comma.
            },
        }
    }
}
