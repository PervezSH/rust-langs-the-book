fn main() {
    ///* String are stored as collections of UTF-8 encoded bytes *///
    /* Create an empty string */
    let empty_string = String::new();
    /* Creat string slices */
    let slicess = "String Slices";
    /* Convert string slice to string */
    let converted_string = slicess.to_string();
    /* Crating string and initializing it */
    let stringg = String::from("String");

    /* Appending to a string */
    let mut s = String::from("foo");
    s.push_str("bar"); // takes string slice, so that we don't take ownership
    s.push('!'); // appends character
    println!("Appended String: {}", s);
    // appending string using + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // here we're moving s1 to s3, then appending all the chars of s2, this save memory compared to copying both strings
    let s3 = s1 + &s2;
    // concatenating using format macro
    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let s6 = format!("{}{}", s4, s5); // it doesn't take ownership

    /* Indexing into a string */
    let hello = String::from("Hello");
    // rust doesn't know what we want to recieve, bytes, scaler, or graapheme clusters, so we can't do
    // let h = hello[0]; // and thus have to use more specific methods
    // access bytes of string
    let mut bytes = hello.bytes();
    assert_eq!(Some(b'H'), bytes.next());
    // access chars of string
    let mut chars = hello.chars();
    assert_eq!(Some('H'), chars.next());
    // to access grapheme cluster, we need to import crate "unicode-segmentation"
}
