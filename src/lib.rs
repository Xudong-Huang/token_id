#![cfg_attr(nightly, feature(macro_reexport))]

#[cfg(nightly)]
#[macro_reexport(TokenId)]
extern crate token_id_plugins;

#[macro_export]
macro_rules! token_id {
    ($t:tt) => {{
        trait TokenId {
            const ID: u64;
        }
        #[derive(TokenId)]
        #[allow(non_camel_case_types)]
        struct $t;
        $t::ID
    }};
}
