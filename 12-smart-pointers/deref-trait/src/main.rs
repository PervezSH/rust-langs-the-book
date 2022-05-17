/// # Deref Trait
/// this allows us to customize the behaviour of dereference behaviour, *
fn main() {
    /* Using box like a reference */
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    /* Creating our own smart pointer */
    use std::ops::Deref;
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }
    let p = 10;
    let r = MyBox::new(p);
    assert_eq!(10, *r);
    // without deref trait, the compiler only knows how to deref references

    /* Implicit Deref Coercions with Functions and Methods */
    // this happens automatically for the types that implements deref trait
    fn hello(arg: &str) {
        println!("{}'s implicit deref coercions", arg);
    }
    let s = MyBox::new(String::from("Rust"));
    hello(&s);
    // &MyBox<String> -> &String -> &str
    hello(&(*s)[..]); // without automatic coercions
}
