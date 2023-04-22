use proc_macro::TokenStream;
use quote::quote;

// This attribute means we're implementing the `derive(HelloMacro)` here.
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // The `quote` macro takes some Rust code and is able to insert values from the macro.
    // The `#variable` syntax seems to be something specific to the `quote` macro.
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}