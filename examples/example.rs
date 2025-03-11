use display_derive::*;

#[derive(MyDisplay)]
struct S {
    a: u64,
    b: String,
}

fn main() {
    let s = S { a: 5, b: "hello".into() };
    println!("{}", s);
}
