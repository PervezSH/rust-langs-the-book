fn main() {
    // creating a vector and inilializing it at the same time
    let v = vec![1, 2, 3, 4, 5];

    // creating a vector and addinf elements to it
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    // accesing vector at an index
    let second = &v[1];
    println!("The second element of vector v is {}", second);
    // a safer way to access elements (handles index out of bounds)
    match v.get(10) {
        Some(second) => println!("The second element of vector v is {}", second),
        None => println!("There is no element at that index!"),
    }

    // iterating over elements in vector
    println!("Printing all the elements in the vector:");
    for i in &v {
        print!("{} ", i);
    }
    println!();
    // iterating over elements and modifying them
    println!("Printing all the elements in the vector v1 after adding 10:");
    for i in &mut v1 {
        *i += 10; // its dereference operator, using it to get the underlying value
        print!("{} ", i);
    }
    println!();

    // storing different types of data inside vector
    enum Date {
        Digit(i32),
        Text(String),
    }
    let date = vec![Date::Digit(10), Date::Text(String::from("Five"))];
    print!("Printing date only if its digit:");
    match &date[0] {
        Date::Digit(i) => println!(" {}", i),
        _ => println!("Not a Digit!"),
    }
}
