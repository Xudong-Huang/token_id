pub use token_id_plugins::TokenId;

#[macro_export]
macro_rules! token_id {
    ($t:tt) => {{
        trait TokenId {
            const ID: u64;
        }
        #[derive($crate::TokenId)]
        #[allow(non_camel_case_types)]
        struct $t;
        $t::ID
    }};
}

#[cfg(test)]
mod test {
    #[test]
    fn test_token_id() {
        assert_eq!(super::token_id!(aaa), 10827978515143807110);
        assert_eq!(super::token_id!(aaa), 10827978515143807110);
        assert_eq!(super::token_id!(bbb), 5136460749158738142);
    }
}
