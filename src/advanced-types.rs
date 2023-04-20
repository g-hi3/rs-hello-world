// Newtype implementations can be used for something akin to type alias in F#.
//     struct Millimeters(i32);
// Passing a raw i32 value to a function that takes `Millimeters` would cause a compiler error.

// Rust has type alias too, but they work a bit different.
// Instead of creating a type that is identical at runtime, but different at compile time, `Kilometers` is a synonym to i32.
type Kilometers = i32;
// The use case for this is to avoid repetition of long types.
type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    // `str` is a dynamically sized type (DST).
    // This means, the value can have an dynamic size at runtime.
    // Rust does not allow direct use of DSTs.
    // Instead, we need to use a pointer.
    let s1: &str = "Hello there!";

    // The `Sized` trait is automatically implemented for each type whose size is known at compile time.
    // The `Sized` trait is also automatically added as a bound to generic functions.
    //     fn generic<T: Sized>(t: T)
    // By default, generic types only work on types that have a known size at compile time.
    // It's possible to relax this restriction by using this:
    //     fn generic<T: ?Sized>(t: &T)
    // This syntax is only available for the `Sized` traits and no other.
    // When using this syntax, the parameter also has to be a reference.
}

// Rust has a never type that can be used with `!`.
// This function is defined to never return.
// Functions like this are also called diverging functions.
// An example of this is used in the case of let-else.
// The defined behavior of `!` is that it can be coerced into any type.
fn bar() -> ! {
    panic!("at the disco");
}

// In the guessing game, we used a `match` that returned a value in `Ok` case, but `continue` for `Err`.
//     let guess: u32 = match guess.trim().parse() {
//         Ok(num) => num,
//         Err(_) => continue
//     };
// Because `continue` is `!`, the type of `guess` is `u32`.
// This is allowed, because instead of returning a value, the `Err` arm will exit the current scope (probably a loop).
// The same is true for code that panics.