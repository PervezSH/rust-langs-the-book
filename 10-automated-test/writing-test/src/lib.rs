#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn is_square(&self) -> String {
        if self.width == self.height {
            format!("Square")
        } else {
            format!("Rectangle")
        }
    }
}

#[cfg(test)]
mod tests {
    /* Writing Tests */
    use super::*; // bring code into the scope
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 5,
        };
        let smaller = Rectangle {
            width: 7,
            height: 4,
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 5,
        };
        let smaller = Rectangle {
            width: 7,
            height: 4,
        };
        assert!(!smaller.can_hold(&larger));
    }

    /* Testing Equality with the assert_eq! and assert_ne! Macros */
    #[test]
    fn area_is_ten() {
        let rectangle = Rectangle {
            width: 2,
            height: 5,
        };
        assert_eq!(10, rectangle.area())
    }
    #[test]
    fn area_is_not_zero() {
        let rectangle = Rectangle {
            width: 2,
            height: 5,
        };
        assert_ne!(0, rectangle.area());
    }

    /* Custom Failure Messages */
    #[test]
    fn it_is_square() {
        let rectangle = Rectangle {
            width: 2,
            height: 5,
        };
        let result = rectangle.is_square();
        assert!(result.contains("Square"), "It is a {}", result);
    }

    /* Checking for Panics with should_panic */
    #[test]
    #[should_panic]
    fn it_panics() {
        panic!("Panics");
    }
    // Panic for a certain msg
    #[test]
    #[should_panic(expected = "Panics due to code 100")]
    fn it_panics_due_to_code_100() {
        panic!("Panics due to code 100");
    }

    /* Using Result<T, E> in Tests */
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 3 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
