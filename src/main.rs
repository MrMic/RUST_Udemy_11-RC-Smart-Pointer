//          ╭─────────────────────────────────────────────────────────╮
//          │            REFERENCE COUNTING SMART POINTER             │
//          ╰─────────────────────────────────────────────────────────╯

use std::rc::Rc;

enum List {
    Cons(i32, Option<Rc<List>>),
}

// INFO: Rc<T> is a reference-counted smart pointer that enables multiple ownership of the same data
// INFO: and in single-threaded scenarios, it allows for shared ownership of data.
fn main() {
    let a = Rc::new(List::Cons(1, Some(Rc::new(List::Cons(2, None)))));
    println!("Reference count after a: {}", Rc::strong_count(&a));
    {
        let b = List::Cons(3, Some(Rc::clone(&a)));
        let c = List::Cons(4, Some(Rc::clone(&a)));
        println!("Reference count after b + c: {}", Rc::strong_count(&a));
    }
    println!("Reference count after scope: {}", Rc::strong_count(&a));
}
