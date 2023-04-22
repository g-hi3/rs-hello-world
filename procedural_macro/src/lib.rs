// Procedural macros work more like functions in the sense that they don't just match a pattern, but they can also calculate.
// A procedural macro is defined like this.
// Procedural macros can only be defined in library crates.
// This is what the macro is called (use a custom name).
#[some_attribute]
// This is the function definition for a procedural macro (use a custom name).
pub fn some_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
}