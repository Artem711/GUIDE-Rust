use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn threader() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep( Duration::from_secs(1) )
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("values"),
            String::from("the"),
            String::from("second"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep( Duration::from_secs(1) )
        }
    });

    for recieved in rx {
        println!("Got: {}", recieved)
    }
}