use std::{
    sync::mpsc,
    thread::{spawn},
};

fn main() {
    let (tx, rx) = mpsc::channel();

    spawn(move || {
        tx.send(String::from("Hello world!")).unwrap();
    });

    let value = rx.recv().unwrap();
    println!("{}", value);
}