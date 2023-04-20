// Functions coerce to the type `fn` (lowercase).
// This is different to the `Fn` closure trait.
// `fn` is called a function pointer.
// Function pointers implement all three of the closure traits (`Fn`, `FnMut` and `FnOnce`).
// This means you can always pass a function pointer to a function that expects a closure.

enum Status {
    Value(u32),
    Stop
}

fn main() {
    // This is how we can pass function pointers to a function.
    let answer = do_twice(add_one, 5);
    println!("answer is {answer}");

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();

    let list_of_statuses: Vec<Status> = (0u32..20)
        // Enum variants can be used as initializer functions.
        // This is comparable to the C# idiom of
        //     .Select(x => new Y(x))
        .map(Status::Value)
        .collect();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

// This is how a function pointer is used as a parameter.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Function pointers don't have a known size at compile-time.
// This means, returning function pointers must always be wrapped in another type.
//     fn returns_closure() -> Box<dyn Fn(i32) -> i32>