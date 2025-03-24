use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
    
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    let a = MyBox::new(x);

    assert_eq!(5, *a);

    // Deref coercion calls * (a.k.a. deref()) as many times as it needs
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]); // equivalent without default deref coercion
}
