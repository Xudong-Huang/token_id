extern crate proc_macro;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(TokenId)]
pub fn token_id(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // calc the hash of the name
    let name = ast.ident;
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    let id = hasher.finish();

    // Build the impl
    let gen = quote!(
        impl token_id::TokenId for #name {
            const ID: u64 = #id;
        }
    );

    // println!("SS = {:?}", gen);
    // Return the generated impl
    gen.into()
}
