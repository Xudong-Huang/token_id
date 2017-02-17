#![feature(associated_consts)]
#[macro_use]
extern crate tocken_id;

fn main() {
    println!("id = {:?}", tocken_id!(bbb));
    println!("id = {:?}", tocken_id!(bb));
    println!("id = {:?}", tocken_id!(b));
}
