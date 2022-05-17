/// # Drop trait
/// allows us to customize what happens when a value goes out of scope
fn main() {
    /* Code cleanup with drop trait */
    #[derive(Debug)]
    struct CustomSmartPointer {
        data: String,
    }
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("CustomSmartPointer Dropped {}!", self.data);
        }
    }
    let p1 = CustomSmartPointer {
        data: String::from("Pointer 1"),
    };
    println!("{:?} will get dropped here...", &p1);
    // Doing this 👇, will result in error, as this will lead to double free
    /*
        p1.drop();
    */
    // to manually drop, do this instead 👇
    drop(p1); // this is different from the drop method on our custom struct
              // this is provided by rust's standard library
    println!("Dropped before the end of the main");
}
