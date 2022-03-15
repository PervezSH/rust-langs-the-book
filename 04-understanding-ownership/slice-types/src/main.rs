fn main() {
    let s = String::from("Hello World");

    let word = first_word(&s);
    println!("{}", word);

    // String literals are slices, that's why they are immuteable
    let s = "Another word!";
    let word = first_word(s);
    println!("{}", word);

    // array slices works the same as string slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    // asserts that two expressions are equal to each other
    assert_eq!(slice, &[2, 3]);
}

// &str allows us to use the same function on both &String values and &str values
fn first_word(s: &str) -> &str {
    // convert string to an array of bytes
    let bytes = s.as_bytes();
    // iterate over array using iter() method
    for (i, &item) in bytes.iter().enumerate() {
        // first element of enumerate is index, and second element is reference to the element
        //byte literal
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
