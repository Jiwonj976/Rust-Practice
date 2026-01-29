/******************************************************************************
Mutli-threading

*******************************************************************************/
use std::thread;

fn main() {
    let crab1 = thread::spawn(|| {
        println!("hello from crab1");
    });

    let crab2 = thread::spawn(|| {
        println!("hello from crab2");
    });

    crab1.join().unwrap();
    crab2.join().unwrap();
}

