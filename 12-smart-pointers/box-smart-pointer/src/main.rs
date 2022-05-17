/// # Box Smart pointer
/// Smart pointers are like pointers but have extra capabilities like metadata and capabillities
/// Smart pointer implements deref and drop traits
/// Deref trait, allows struct to be treated as references
/// Drop trait, allows to customize code when instance goes out of scope
fn main() {
    /* Using Box Smart Pointer */
    // stores on th heap
    // use box:
    //  when you have a type thats exact size can't be known at compile time
    //  when you have large data and you want to transfer the ownership but you don't want to copy it
    let b = Box::new(5);
    println!("B = {}", b);

    /* Enabling Recursive Types with Boxes */
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
