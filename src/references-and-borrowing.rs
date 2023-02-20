fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    print_same_reference();
    print_readonly_reference();
    print_other_reference();
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s is immutable here, so we can't change it.
    // some_string.push_str(", world"); // Compiler error
    // To make s a mutable reference, use s: &mut String as the parameter definition.
}
// When s goes out of scope, it isn't dropped, because calculate_length does not own it.

fn print_same_reference() {
    let mut s = String::from("hello");

    // This borrow is fine, because we don't use s.
    let _r1 = &mut s;
    // This borrow is illegal, because we (would) use _r1 later.
    let _r2 = &mut s;

    // println!("{}, {}", _r1, _r2);
}

fn print_readonly_reference() {
    // Rust compiler prints a warning when this is mutable, because it doesn't need to be.
    let s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // not allowed

    println!("{} and {}", r1, r2);
}

fn print_other_reference() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// A function returning a reference is not valid.
// fn dangle(s: &String) -> &String {
//     &s
// }