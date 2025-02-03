use std::io;

fn main() {
    twelve_days_of_christmas();
    println!("Hello, world!");
    let (celcius, farenheit) = celcius_to_farenheit();
    println!("Entered {celcius}C -> {farenheit}F");

    let fib = fib(celcius as i32);
    println!("{fib}")
}

fn fib(n: i32) -> i32 {
    let (mut lo, mut hi) = (1, 1);
    for _ in 0..n - 2 {
        let tmp = hi;
        hi += lo;
        lo = tmp;
    }
    hi
}

fn celcius_to_farenheit() -> (f32, f32) {
    println!("Enter Celcius:");
    let mut celcius = String::new();
    io::stdin()
        .read_line(&mut celcius)
        .expect("Not a valid number");
    let celcius: f32 = match celcius.trim().parse() {
        Ok(x) => x,
        Err(e) => panic!("Panicked error: {e}"),
    };
    (celcius, celcius * 1.8 + 32.0)
}

fn twelve_days_of_christmas() {
    let gifts = [
        "a partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    for day in 0..11 {
        println!("On the {} day of Christmas my true love gave to me", day+1);
        for i in (0..day).rev() {
            let gift = gifts[i];
            if i==0 && day>0 {
                print!("And ")
            }
            println!("{gift}");

        }
        println!();

    }
}
