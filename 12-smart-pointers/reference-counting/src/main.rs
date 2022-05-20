/// # Reference Counted Smart Pointer
/// to enable multiple ownership of value, we can use reference counting smart pointer, Rc<T>
/// we can use it when we want to allocate data on heap fot multiple parts of our data
fn main() {
    /* Using Rc<T> to share data */
    // we'll get an error if we compile this 👇
    /*
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
        use List::{Cons, Nil};
        let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        let b = Cons(3, Box::new(a)); // a is moved into b
        let c = Cons(4, Box::new(a));
    */
    // so instead we can change the defination of List
    enum List {
        Cons(i32, Rc<List>), // make List use reference counting pointer instead of box counter
        Nil,
    }
    use std::rc::Rc; // bring reference counting smart counter into scope
    use List::{Cons, Nil};
    // wrap it inside reference counting smart pointer, as we need to pass it
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); // construct instances of reference counting
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    // Rc::clone doesn't make deep copies, it only increasing the reference count

    /* Illustrate how cloning an Rc<T> increases the reference count */
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
