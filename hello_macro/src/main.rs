use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

struct Pancakes;


impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Mello, Macro! My name is Pancakes! This is an impl.");
    }
}

#[derive(HelloMacro)]
struct Waffles;

fn main() {
    Pancakes::hello_macro();
    Waffles::hello_macro();
}