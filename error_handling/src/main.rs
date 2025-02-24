use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::fs;
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    // The Box<dyn Error> type is a trait object, will be explained later

    println!("Hello, world!");
    // panic!("AAAARG Please Panic Now");

    let greeting_file_result = File::open("hello.txt");
    println!("{:?}", greeting_file_result);
    let greeting_file = match greeting_file_result {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem opening the file: {e:?}"),
            },
            other_error  => {
                panic!("Problem opening the file: {other_error:?}")
            }
        },
    };
    alternative_with_closures();

    // Unwrap panics on error
    let greeting_file = File::open("hello.txt").unwrap(); 
    // Expect panics on error with an error message
    let greeting_file = File::open("hello.txt")
            .expect("We"); 
    Ok(())
}

fn alternative_with_closures() {
    // This example is the same as main, but uses "closures" 
    //  instead to reduce the number of match expressions
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

// Propogating errors by using result and returning Err
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Now with question mark operator to propogate error. They go through From trait
fn read_username_from_file_with_question() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}


fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_one_line() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}


fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
