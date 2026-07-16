use std::{
    sync::mpsc,
    thread::{spawn},
};

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let producer = tx.clone();
        spawn(move || {
            let mut sum: u64 = 0;
            for j in i * 10000000 + 1..((i+1) * 10000000) {
                sum += j;
            }
            producer.send(sum).unwrap();
        });
    }

    drop(tx);

    let mut result = 0;
    for val in rx {
        println!("Receive value from thread");
        result += val;
    }

    println!("Final sum: {}", result);
}