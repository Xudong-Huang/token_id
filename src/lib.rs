#![feature(associated_consts)]
#![feature(macro_reexport)]

#[macro_reexport(TockenId)]
extern crate tocken_id_plugins;

pub trait TockenId {
    const ID: u64;
}

#[macro_export]
macro_rules! tocken_id {
    ($t:tt) => {{
        use $crate::TockenId;
        #[derive(TockenId)]
        #[allow(non_camel_case_types)]
        struct $t;
        $t::ID
    }};
}
