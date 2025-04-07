use std::sync::{Arc, Mutex};
use std::thread;
use std::rc::Rc;

fn main_single_thread() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
        // the Drop implementation for MutexGuard releases the lock
    }

    println!("m = {m:?}");
}


fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            println!("Thread: Incrementing {counter:?}");
            let mut num = counter.lock().unwrap();
            println!("Thread: Incrementing {num}");

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}