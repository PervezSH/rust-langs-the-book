/* Defining a trait for common behaviour */
// Using Trait Objects That Allow for Values of Different Types
// define a public trait draw
pub trait Draw {
    fn draw(&self);
}
// define struct that will hold list of drawable components
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // vector containing a box smart pointer containing any type that implements Draw trait
}
// create implementation block for Screen struct
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
// implementing the Trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        // draw
    }
}
