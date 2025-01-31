fn main() {
    println!("Hello, world!");

    another_function(3, 'h');

    let y = {
        let x = 3;
        x + five()
    };

    println!("The value of y is: {y}");
}

fn another_function(x: i32, unit_label: char) {
    println!("The measurement is: {x}{unit_label}");
}

fn five() -> i32 {
    5
}