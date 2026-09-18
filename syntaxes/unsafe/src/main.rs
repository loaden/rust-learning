mod raw_pointer;
use crate::raw_pointer::raw_pointer_test;

mod state;
use crate::state::Post;

fn main() {
    let mut p = Post::new();
    p.add_text(" world");
    println!("{}", p.content());

    raw_pointer_test();
}
