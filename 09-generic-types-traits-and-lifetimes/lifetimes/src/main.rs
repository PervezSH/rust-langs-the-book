fn main() {
    /* Dangling References */
    // reference that points to invalid data
    /*
        let r;
        {
            let x = 5;
            r = &x;
        } // `x` dropped here while still borrowed
        println!("r: {}", r);
    */
    // rust knows this at compile time, using borrow checker
    // Borrow checker runs at compile time and make sure that all borrowed value are valid

    /* Generic Lifetimes */
    let string1 = String::from("abcd");
    let string2 = "xyz";
    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    let result = longest(string1.as_str(), string2);
    // lifetime of result is the smallest lifetime being passed in
    println!("The longest string is {}", result);
    // Example where strings has different lifetimes 👇
    /*
        let string3 = String::from("abcd");
        let result1;
        {
            let string4 = String::from("xyz");
            result1 = longest(string3.as_str(), string4.as_str()); // lifetime of result1 is that of string4
        } // string4 gets dropped here and so does our result1
        println!("The longest string is {}", result1); //dangling reference
    */

    /* Lifetime Annotations in Struct  */
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important Excerpt {:?}", i.part);

    /* Lifetime Elision */
    // Set of particular cases that compiler will consider, and we don't have to write the lifetimes explicitly
    // 1. Each parameter that is a reference gets its own lifetime
    // 2. If there is exactly one input lifetime, that lifetime is assigned to output lifetime
    // 3. If there are multiple input lifetime, but one of them is `self`, life of `self` is assigned to output lifetime

    /* Static Lifetime */
    // the refernce could live as long as the duration of the program, all String literals have static lifetime
}
