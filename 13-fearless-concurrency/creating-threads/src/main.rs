/// # Creating Threads
/// Threads: the feature that runs independent parts of program simultaneously within a program
use std::{thread, time::Duration};
fn main() {
    /* Creating new thread using spawn */
    // we call spwan function and pass in a closure to creat a new thread
    /*
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1)); // force a thread to stop its execution for a short duration
            }
        }); // no matter if it finish executing or not, it will stop when the main thread ends,
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    */

    /* Modifying to allow spawn thread finish execution */
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1)); // force a thread to stop its execution for a short duration
        }
    });
    // if we call .join() here, main thread will wait for the spawned thread to finish
    // handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); //.join() will make sure the spawned thread finishes its execution

    /* Using move Closures with Threads */
    // untill this point, thread didn't depend on any variables outside the thread
    let v = vec![1, 2, 3];
    // bcoz rust doesn't know how long our threead will run for, it won't allow us to take reference of v inside the closure
    // so we need force the closure to take ownership of the values it’s using with move keyword
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}
