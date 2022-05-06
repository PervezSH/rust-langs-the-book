use std::fmt::Display;

fn main() {
    /* Defining a Traits */
    // traits are used to define a shared behaviour
    pub trait Summary {
        // here we define the shared methods
        fn summerize(&self) -> String;
        // default implementation
        fn summerize_default(&self) -> String {
            String::from("Default Summary!")
        }
    }

    /* Implementing Trait */
    pub struct NewsArticle {
        pub author: String,
        pub headline: String,
        pub content: String,
    }
    impl Summary for NewsArticle {
        fn summerize(&self) -> String {
            format!("{} , by {}", self.headline, self.author)
        }
    }
    let news = NewsArticle {
        author: String::from("@Me"),
        headline: String::from("Traits"),
        content: String::from("Understanding rust traits"),
    };
    println!("Article summary : {}", news.summerize());
    // implementing default summary
    println!("{}", news.summerize_default());

    /* Traits as a Parameter */
    pub fn notify(item: &impl Summary) {
        println!("New Summary! {}", item.summerize())
    }
    // ☝️ is actually syntax sugar for Trait Bound Syntax, which looks like this 👇
    /*
        pub fn notify<T: Summary>(item: &T) {
            println!("New Summary! {}", item.summerize())
        }
    */
    notify(&news);

    /* Clearer Traits Bounds with Where Clause */
    // instead of writing this 👇
    /*
        fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    */
    // we can write this 👇
    /*
        fn some_function<T, U>(t: &T, u: &U) -> i32
            where T: Display + Clone,
                U: Clone + Debug
        {}
    */

    /* Returning Types that Implements Traits */
    // it is very usefull in the context of Closures and Iteraetor
    // you can it for only returning one single type
    fn return_article() -> impl Summary {
        NewsArticle {
            author: String::from("@return_article"),
            headline: String::from("Traits"),
            content: String::from("Returning types that implements Traits"),
        }
    }
    println!("New Summary! {}", return_article().summerize());
}

/* Using Trait Bounds to Conditionally Implement Methods */
struct Pair<T> {
    x: T,
    y: T,
}
// 👇 implementation block is for any Pair struct
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
// 👇 implementation block is only available to Pair structs, where
// the types of x and y implements Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest is x = {}", self.x);
        } else {
            println!("The largest is y = {}", self.y);
        }
    }
}
