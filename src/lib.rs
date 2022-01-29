pub use token_id_plugins::TokenId;

pub trait TokenId {
    const ID: u64;
}

#[macro_export]
macro_rules! token_id {
    ($t:tt) => {{
        #[derive($crate::TokenId)]
        #[allow(non_camel_case_types)]
        struct $t;
        <$t as $crate::TokenId>::ID
    }};
}
