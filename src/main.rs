//          ╭─────────────────────────────────────────────────────────╮
//          │            REFERENCE COUNTING SMART POINTER             │
//          ╰─────────────────────────────────────────────────────────╯

enum List {
    Cons(i32, Option<Box<List>>),
}

fn main() {
    println!("Hello, world!");
}
