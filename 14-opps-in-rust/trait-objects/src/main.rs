/// # Trait Objects
/// in rust, shared behaviour is defined using traits
// import the draw trait
use trait_objects::{Button, Draw, Screen};
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        // draw select box
    }
}
fn main() {
    /* Defining a trait for common behaviour */
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 10,
                height: 20,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
                ],
            }),
            Box::new(Button {
                width: 20,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
    // because we used trait objects rust will ensure at compile time that any component in the list implements draw trait
}
