extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use proc_macro::TokenStream;

#[proc_macro_derive(TockenId)]
pub fn tocken_id(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // calc the hash of the name
    let name = &ast.ident;
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    let id = hasher.finish();

    // Build the impl
    let gen = quote! {
        impl TockenId for #name {
            const ID: u64 = #id;
        }
    };

    // println!("SS = {:?}", gen);
    // Return the generated impl
    gen.parse().unwrap()
}
