/// * Shared State
/// with shared state concurrency multiple threads can acces the same memory location at the same time
use std::sync::{Arc, Mutex}; // import mutex from the standard library
use std::thread;
fn main() {
    /* Using Mutex to allow access to data */
    // its an abbreviation for mutual exclusion
    // it allows only one thread to access data at any given time, to achive this  mutex use a locking system
    // to access data in a mutex, the thread must first signal that it wants access by asking to acquire the mutex's lock
    // lock is a data structure that keeps track of who currently has exclusive access to the data
    // two rules that you need to remember:
    // 1. you must acquire the lock before using the data
    // 2. when you're done you must unlock the data so that other thread can use it
    // create mutex
    let m = Mutex::new(5);
    {
        // accessing the data in mutex
        let mut num = m.lock().unwrap();
        // updating the underlying data of the mutex
        *num = 10;
    } // mutex guard implement the drop trait, so here it will release the lock to the data
    println!("m : {:?}", m);

    /* Sharing a Mutex<T> Between Multiple Threads */
    // We’ll spin up 10 threads and have them each increment a counter value by 1
    // So the counter goes from 0 to 10
    // wrap it ARC smart pointer so that it can have multiple owners
    // Arc is like Rc except that it can be shared accross threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    // spin up 10 threads and have them increment the counter
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    // make sure all the threads finishes their execution
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
