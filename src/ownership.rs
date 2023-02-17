fn main() {
    // This is a string literal type.
    let _x: &str = "";
    // This is a mutable string created from a string literal.
    let mut s = String::from("hello");
    // This function appends the string literal to the string.
    s.push_str(" world");
    // String literals are not the same type as String.
    // The println macro can't be called with a string variable.
    println!("{}", s);

    {
        let _s = String::from("hello"); // _s is valid from this point forward

        // do stuff with _s
    }                                  // this scope is now over, and _s is no longer valid

    let number_a = 5;
    // At this point, Rust copies the value of number_a and assigns the copy to _number_b.
    // This is because numbers are simple values that are pushed to the stack.
    let _number_b = number_a;

    // Strings consist of:
    // ptr -> pointer to the heap with all the string data
    // len -> length of the string in bytes
    // capacity -> total amount of memory the allocater provides (in bytes)

    let s1 = String::from("hello");
    // When we do this, the data (ptr, len, capacity).
    // The data on the heap remains, making the string a new value using the same reference.
    // This also makes s1 invalid.
    // Uses of s1 after this point will generate a compiler error.
    let s2 = s1;
    // To make a real copy of a string, the clone function can be used.
    let _s3 = s2.clone();

    // Rust has a Copy trait that can be placed on types that are stored on the stack.
    // This makes variables of that type still valid after a copy.
    // There is also a Drop trait that causes the inverse effect.
    // Rust generates a compiler error for any type that implements the Copy trait and contains a value that implements the Drop trait.
    // Tuples of the same type implement the Copy trait, but Tuples of different types do not.
}