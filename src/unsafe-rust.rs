// The `extern` keyword allows us to import functions using a foreign function interface (FFI).
// In this case, we use the C application binary interface (ABI) to import `abs`.
extern "C" {
    fn abs(input: i32) -> i32;
}

// A global variable in Rust is called `static`.
// Accessing mutable static variables in multithreaded contexts, it may cause a data race.
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

// Traits and impl blocks may be marked as `unsafe`.
// Functions defined in unsafe traits may only be called in `unsafe` scopes.
unsafe trait Foo {
}
unsafe impl Foo for i32 {
}

// `union`s are comparable to structs, but only one field is used in a particular instance at one time.
// They're primarily used to interact with C code that uses unions.
union Something {
    x: i32,
    y: i32
}

fn main() {
    let mut num = 5;
    // Raw pointers are memory safe and can be created outside of an `unsafe` block.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // An unsafe block enables "unsafe superpowers".
    unsafe {
        // In this scope we're allowed to:
        //  * Dereference a raw pointer.
        //  * Call an unsafe function or method.
        //  * Access or modify a mutable static variable.
        //  * Implement an unsafe trait.
        //  * Access fields of `union`s.
        // It does not disable any borrow checker features.

        // Raw pointers use the types `*const T` and `*mut T`.
        // They are different from references and smart pointers:
        //  * allowed to ignore borrowing rules
        //  * aren't guaranteed to point to valid memory
        //  * allowed to be null
        //  * don't implement any automatic cleanup

        // Dereferencing a raw pointer.
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        // Unsafe functions or methods may only be called in `unsafe` blocks and functions.
        dangerous();

        // Functions from the FFI are only allowed to be called within an `unsafe` block or function.
        abs(32);

        // Using mutable static variables must be done in `unsafe` scopes.
        COUNTER += 10;

        // Accessing a field of a union is only allowed in unsafe scopes.
        let x = Something { x: 10 };
        println!("Something's x: {}", x.x);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    // `split_at_mut` is a safe function that requires an unsafe block.
    // Rust doesn't know what we want two references to the same slice that don't overlap.
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// Functions and methods may be marked unsafe.
// This effectively makes the body of the function be an unsafe block.
unsafe fn dangerous() {
}

// The `no_mangle` attribute tells the compiler not to mangle the function name.
// Mangling is when the compiler adds more context-specific information to a function name that makes it less human-readable.
#[no_mangle]
// Adding an `extern` to the function definition makes it available in another language.
// Calling this function is allowed from non-`unsafe` scopes.
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}