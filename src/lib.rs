#![feature(associated_consts)]
#![feature(macro_reexport)]

#[macro_reexport(TokenId)]
extern crate token_id_plugins;

pub trait TokenId {
    const ID: u64;
}

#[macro_export]
macro_rules! token_id {
    ($t:tt) => {{
        use $crate::TokenId;
        #[derive(TokenId)]
        #[allow(non_camel_case_types)]
        struct $t;
        $t::ID
    }};
}
