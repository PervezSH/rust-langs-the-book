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
    // associated function that don't have self
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn is_square(&self) -> bool {
        self.height == self.width
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

    let rect3 = Rectangle::square(10);
    println!("Is rect3 a square? {}", rect3.is_square());
}
