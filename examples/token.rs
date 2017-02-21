#![feature(associated_consts)]
#[macro_use]
extern crate token_id;

fn main() {
    println!("id = {:?}", token_id!(bbb));
    println!("id = {:?}", token_id!(bbb));
    println!("id = {:?}", token_id!(bb));
    println!("id = {:?}", token_id!(b));
}
