/******************************************************************************
Channels

*******************************************************************************/
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let another_thread = thread::spawn(move || {
        let v = rx.recv().unwrap();
        println!("Thread received: {}", v);
    });

    tx.send(1).unwrap();
    drop(tx);

    another_thread.join().unwrap();
}

