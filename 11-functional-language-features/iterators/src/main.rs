fn main() {
    /* Processing series of items with iterators */
    // iterator allows to interate over a sequence of elements
    // iterators are lazy, meaning they have no effect untill you call methods that consums the iterator
    let v1 = vec![1, 2, 4];
    let v1_iter = v1.iter();
    for value in v1_iter {
        print!("{}, ", value);
    }
    // all iterators in rust implements the iterator trait
    // the defination looks like this 👇
    /*
        pub trait Iterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
            // methods with default implementations elided
        }
    */
    let v2 = vec![1, 2, 4];
    let mut v2_iter = v2.iter(); // immutable references
                                 // let mut v2_iter = v2.iter(); // mutable references
                                 // let mut v2_iter = v2.into_iter(); // owns
    assert_eq!(v2_iter.next(), Some(&1));
    assert_eq!(v2_iter.next(), Some(&2));
    assert_eq!(v2_iter.next(), Some(&4));
    assert_eq!(v2_iter.next(), None);

    /* Methods that comsumes the iterator */
    let total: i32 = v2_iter.sum();
    assert_eq!(total, 7);

    /* Methods that produces other iterators */
    // knowns as iterator adaptors, allows us to change interators into different iterators
    let v3 = vec![1, 2, 3];
    let modified_v3: Vec<_> = v3.iter().map(|x| x + 1).collect();
    assert_eq!(modified_v3, vec![2, 3, 4]);

    /* Iterators with closures that captures the environment */
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }
    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }
    let shoe = Shoe {
        size: 10,
        style: String::from("Boot"),
    };
    assert_eq!(
        shoes_in_size(vec![shoe], 10),
        vec![Shoe {
            size: 10,
            style: String::from("Sneaker")
        }]
    );

    /* Implementing our own iterator */
    struct Counter {
        count: u32,
    }
    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }
    impl Iterator for Counter {
        type Item = u32; // associative type
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    /* Using Other Iterator Trait Methods */
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1)) // zip() takes two interator and zip them up into 1
        // skip() creats iterator, skipping first n elements
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
