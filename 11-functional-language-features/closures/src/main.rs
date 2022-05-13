fn main() {
    /* Defining a Closure */
    // closures are anonymous function, can be saved in a variable or pass as arguments
    let example_closure = |num1, num2| {
        println!("Hello, from closure");
        num1 + num2
    }; // closure contains the defination of anonymous function not the resulting value
    let three = example_closure(1, 2); // We call a closure like we do a function
    println!("Calculated using closure, three = {}", three);

    /* Closure Type Inference and Annotation */
    // closures don't require you to annotate type
    // the first type that is passed into the closure will be concrete type
    let example_closure_1 = |x| x;
    let _n = example_closure_1(5);
    // doing this will throw an error 👇
    // let s = example_closure_1(String::from("Hello"));

    /* Storing Closures Using Generic Parameters and the Fn Traits */
    // closure inside a struct need to specify type, just like a field inside a struct needs to know its type
    // to define a closure inside a struct, enum, or function, we use generics and trait bounds
    // closure implement atleast one of the traits: Fn FnMut, FnOnce
    struct ClosureStruct<T>
    where
        T: Fn(u32, u32) -> u32,
    {
        closure: T,
    }
    impl<T> ClosureStruct<T>
    where
        T: Fn(u32, u32) -> u32,
    {
        fn new(_closure: T) -> ClosureStruct<T> {
            ClosureStruct { closure: _closure }
        }
    }
    let closure_struct = ClosureStruct::new(|num1, num2| {
        println!("Hello, world, from closure struct");
        num1 + num2
    });
    // calling closure inside a struct
    let five = (closure_struct.closure)(2, 3);
    println!("Calculated using closure, three = {}", five);

    /* Capturing the environment with closure */
    // closure can capture their environment, and they use extra memory to do so, whereas function can't and don't
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
    // closure  captures value from its environment in three ways:
    // taking ownership, borrowing mutably, and borrowing immutably
    // encode in Fn traits: FnOnce, FnMut, Fn
}
