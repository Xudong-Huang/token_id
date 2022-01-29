use token_id::{TokenId, token_id};

#[derive(TokenId)]
struct A {
    _a: u32
}

fn main() {
    // let _ = token_id!(bbb);
    println!("id = {:?}", token_id!(bbb));
    println!("id = {:?}", token_id!(bbb));
    println!("id = {:?}", token_id!(bb));
    println!("id = {:?}", token_id!(b));
    println!("id = {:?}", <A as TokenId>::ID);
}
