/// * Reference Cycles
/// rust makes it difficult but not immposible to create memory that is never cleaned up
/// we can create memory leak using Rc and RefCell smart pointer

fn main() {
    /* Create a Reference Cycle */
    use std::cell::RefCell;
    use std::rc::Rc;
    use List::{Cons, Nil};
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>), // Rc smart pointer, so that we can have multiple owners
        // RefCell smart pointer, so that we can mutate the list
        Nil,
    }
    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a)); // 1
    println!("a next item = {:?}", a.tail()); // Nil
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a)); // 2
    println!("b initial rc count = {}", Rc::strong_count(&b)); // 1
    println!("b next item = {:?}", b.tail()); // list a

    // modifying list a to store list b
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // 2

    // we just created a reference cycle
    // Uncomment the next line to see that we have a reference cycle;
    // it will overflow the stack
    /*
        println!("a next item = {:?}", a.tail());
    */

    /* Preventing Reference Cycles: Turning an Rc<T> into a Weak<T> */
    // Creating a Tree Data Structure: a Node with Child Nodes
    use std::rc::Weak;
    #[derive(Debug)]
    struct Node {
        value: i32,
        // a parents can own children, however the same is true for children, children should not own parent
        // so that parent doeesn't get dropped, bcoz of this relationship we can use weak smart pointer
        parent: RefCell<Weak<Node>>, // here, we can't use Rc smart as this will create reference cycle
        children: RefCell<Vec<Rc<Node>>>,
    }
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    // upgrade() transforms Weak smart pointer to Rc smart pointer
    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade()); // None
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    // now we'll modify leaf's parent field to store our branch node
    // method downgrade() transforms Rc smart pointer to Weak smart pointer
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade()); // Branch

    /* Visualizing Changes to strong_count and weak_count */
    // Rc smart pointers stores two counts: strong_count and weak_count
    // strong_count: no. of reference which have ownership of the data
    // weak_count: no. of reference which doesn't have ownership of the data
    println!(
        "leaf strong = {}, branch weak = {}",
        Rc::strong_count(&leaf), // 2
        Rc::weak_count(&branch), // 1
    );
}
