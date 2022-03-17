#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    // create an instance of rectangle
    let rect1 = Rectangle {
        width: 80,
        height: 20,
    };
    println!("Rect1 is {:?}", rect1);
}
