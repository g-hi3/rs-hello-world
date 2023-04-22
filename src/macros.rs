// Rust has declarative macros and three kinds of procedural macros.
//  * #[derive] macros that add code to structs and enums
//  * attribute-like macros
//  * function-like macros

// Macros generate code at compile time to reduce the amount of code we need to write.
// In addition, macros can take a variable number of parameters (such as `println` with multiple args).
// The downside to this is that macros are more complex to write.
// Also, macros must be brought into scope or defined before they are used, unlike functions than can be declared and used anywhere.

// Declarative macros are most widely used and they're also called `macro_rules!` or macros by example.
// They match some Rust code against a pattern.

// This annotation indicates that the macro will be available whenever the crate is brought into scope.
// Without it, it would only be available to the current crate.
#[macro_export]
// This line defines the name of the macro (without the `!`).
macro_rules! custom_vec {
    // This is an arm with a macro pattern.
    // If the pattern matches, this expression will be used.
    // If none of the patterns match, there will be an error.
    // The `$` marks a macro variable and keeps it unique from actual Rust variables.
    // The comma is a literal comma that may follow the expression.
    // The `*` means that there may be zero or more matches of whatever precedes it.
    ( $( $x:expr ),* ) => {
        // This code is inserted whenever the macro is called.
        {
            let mut temp_vec = Vec::new();
            // The following code is inserted once for each time `*` matched.
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Attribute-like macros work on other items than structs and traits as well, such as functions.
// An example is the `#[route()]` macro.
// It works like this:
//     #[route(GET, "/")]
//     fn index() {}
// That macro would be declared like this:
//     #[proc_macro_attribute]
//     pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream
// The first parameter is the content of the attribute (so `GET, "/"`).
// The second is the item that it is attached to, so the function `pub fn index() {}`.

// Function-like macros work much the same way as declarative macros.
// The difference is that they take a `TokenStream` and generate code from it.
//     let sql = sql!(SELECT * FROM posts WHERE id = 1);
// Would be defined as such:
//     #[proc_macro]
//     pub fn sql(input: TokenStream) -> TokenStream { ... }

fn main() {
    let my_vector = custom_vec![1, 5, 9];
}