// mpsc: multiple producer, single consumer
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // tx = Transmitter, rx = Receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // recv = Receive
    let received = rx.recv().unwrap();
    println!("Got: {received}");


    // Multiple messages
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
        
        thread::sleep(Duration::from_millis(500));
        tx.send(String::from("Fin.")).unwrap();
    });

    for received in rx {
        println!("Got: {received}");
    }
    handle.join().unwrap();


    // Multiple producers created by cloning tx   
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

    for received in rx {
        println!("Got: {received}");
    }

}