struct User {
    username: String,
    email: String,
    // A trailing comma in the definition of a struct is allowed in Rust.
    sign_in_count: u64,
}

fn main() {
    let mut user1 = build_user(
        String::from("someemail@example.com"),
        String::from("someuser123"));

    // If the variable is mutable, we can assign a field using the dot operator like in Java or C#.
    // Rust does not allow marking only certain fields as mutable.
    user1.email = String::from("anotheremail@example.com");

    let _user2 = User {
        email: String::from("another@example.com"),
        // The update syntax allows us to copy the remaining fields from an existing variable.
        // This expression must come last in the assignments.
        ..user1
    };

    // We can no longer use user1.username, because it has been moved to _user2.username.
    // Because active and sign_in_count use types that implement the Copy trait,
    // their values have been copied instead.

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // _black and _origin are objects of different types.
    // Tuple structs are not interchangeable, even if their field definitions look identical.

    let _subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    // Rust displays an error if any fields are not assigned.
    User {
        // The order of the definition and the assignments do not need to match.
        sign_in_count: 1,
        // When a variable has the same name and type as a field, the assignment can be simplified.
        // This is called field init shorthand syntax.
        username,
        // As with the definition, the trailing comma is allowed in the assignment.
        email,
    }
}

// Rust also has structs that look similar to tuples, called tuple structs.
// They use unnamed, ordered fields.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// It's also possible to create structs without fields.
// The book explains, that we might want to implement some behaviour later using traits.
struct AlwaysEqual;