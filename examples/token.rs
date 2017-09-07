#[macro_use]
extern crate token_id;
#[macro_use]
extern crate token_id_plugins;

fn main() {
    println!("id = {:?}", token_id!(bbb));
    println!("id = {:?}", token_id!(bbb));
    println!("id = {:?}", token_id!(bb));
    println!("id = {:?}", token_id!(b));
}
