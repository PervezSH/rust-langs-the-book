/// * Message Passing
/// used to transfer data between threads
use std::sync::mpsc; // mpsc stands for multiple producer, single consumer
                     // means a channel can have multiple sending ends that produce values
                     // but only one receiving end that consumes those values
use std::thread;
use std::time::Duration;
fn main() {
    /* Pass Msg from one thread to other */
    // create a channel
    let (tx, rx) = mpsc::channel();
    // spawn a thread and use sender to send message
    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap(); // if the receiving end drops, then send will return an error
    });
    let received = rx.recv().unwrap(); // recv() blocks the main thread execution and wait, try_recv() doesn't block
    println!("Got: {}", received);
    println!("****************************");

    /* Sending Multiple Values and Seeing the Receiver Waiting */
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
    println!("****************************");

    /* Creating Multiple Producers by Cloning the Transmitter */
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // msgs from each thread will be send non deterministically
    for received in rx {
        println!("Got: {}", received);
    }
}
