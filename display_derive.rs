use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(MyDisplay)]
pub fn my_display_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct
    let name = &input.ident;

    // Check if the struct has named fields
    let fields = if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            fields.named.iter()
        } else {
            panic!("MyDisplay can only be derived for structs with named fields");
        }
    } else {
        panic!("MyDisplay can only be derived for structs");
    };

    // Generate the format string and the field accessors
    let format_string = fields.clone().map(|f| {
        let field_name = &f.ident;
        format!("{}: {{}} ", field_name.as_ref().unwrap())
    }).collect::<Vec<_>>().join(", ");

    let field_accessors = fields.map(|f| {
        let field_name = &f.ident;
        quote! { &self.#field_name }
    });

    // Generate the implementation of Display
    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, #format_string, #(#field_accessors),*)
            }
        }
    };

    // Return the generated implementation as a TokenStream
    TokenStream::from(expanded)
}