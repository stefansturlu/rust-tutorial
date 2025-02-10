#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:#?}!");
            25
        }
    }
}



fn main() {
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    // Matching with Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Six: {}", six.expect("Should be 6"));
    //println!("None: {}", none.expect("Should be none and panic"));

    // Catch-all patterns and _
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    for d in 2..8 {
        match d {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            eitthvad => move_player(eitthvad),
        }
    }


    fn add_fancy_hat() { println!("Fancy hat added!") }
    fn remove_fancy_hat() { println!("No fancy hat for you!") }
    fn move_player(num_spaces: u8) { println!("Moved {num_spaces} spaces") }


    // Ownership of match
    let opt: Option<String> = Some(String::from("Hello world"));
    
    // opt became &opt. If & is removed, then it won't compile
    match &opt {
        Some(s) => println!("Some: {}", s),
        None => println!("None!")
    };
    
    println!("{:?}", opt);

}
