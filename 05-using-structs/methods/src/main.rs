#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // method with parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 80,
        height: 20,
    };
    let rect2 = Rectangle {
        width: 70,
        height: 10,
    };

    println!("Area of rect1 is {}", rect1.area());
    println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));
}
