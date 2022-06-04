/// # RefCell<T> and the Interior mutability pattern
/// it allows us to mutate data even when there is immutable reference to that data
/// this pattern uses unsafe code to bypass the typical rule on mutation
/// unsafe code is not checked at compile time
fn main() {
    /* Interior Mutability: A Mutable Borrow to an Immutable Value */
    // mutating the value inside an immutable value is the interior mutability pattern
    // this 👇 will throw error, coz we can not 'x' as mutable as it is not declared as mutable
    /*
        let x = 5;
        let y = &mut x;
    */

    /* A Use Case for Interior Mutability */
    use std::cell::RefCell;
    pub trait Messenger {
        fn send(&self, msg: &str);
    }
    // to use interior mutability wrap sent_messages inside the RefCell<T>
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        // here we're taking immutable reference of self
        fn send(&self, message: &str) {
            // here we're mutating the vector
            self.sent_messages.borrow_mut().push(String::from(message)); // to get mutable reference to the value call borrow_mut()
        }
    }
    let mock_messenger = MockMessenger::new();
    mock_messenger.send("msg");
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1); // to get immutable reference call borrow()

    /* Combining Rc<T> and RefCell<T> to get multiple owners of mutable data */
    #[derive(Debug)]
    enum List {
        // both the value and list are wrapped in Rc smart pointer, coz we want them to have multiple owners
        // integer value is wrapped in RefCell smart point, coz we want themm to be mutable
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }
    use std::rc::Rc;
    use List::{Cons, Nil};
    // creating value by wrapping it in a RefCell and Rc smart pointer
    // so that we can mutate it and it have multiple owners
    let value = Rc::new(RefCell::new(5));
    // creating first cons list
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    // derefence Rc into RefCell, and then mutating it by calling borrow_mut()
    *value.borrow_mut() += 10;
    // at this point integer store inside value should be 15
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
