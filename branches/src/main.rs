fn main() {
    let mut number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
