fn main() {
    /* Creating a Generic Function */
    let num_list = vec![3, 6, 1, 0];
    println!("Largest is {}", get_largest(num_list));
    let char_list = vec!["a", "z", "b"];
    println!("Largest is {}", get_largest(char_list));
    // PartialOrd, and Copy are traits, that restricts generic types
    fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
        let mut largest = list[0];
        for i in list {
            if i > largest {
                largest = i;
            }
        }
        largest
    }

    /* Generics with structs */
    struct Point<T, U> {
        x: T,
        y: U,
    }
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let p3 = Point { x: 5, y: 10.0 };
}
