// Recursive type. We need Box, otherwise we don't know stack size at compile time
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Non-recursive type. Has known max size at compile time
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

use crate::List::{Cons,Nil};

fn main() {
    // Box: stores value on heap. Stack has address in heap.
    let b = Box::new(5);
    println!("b = {b}");
    // Using recursive type
    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(5, Box::new(Nil))))));
}
