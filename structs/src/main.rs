struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Tuple structs (non-named fields)
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn using_tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// Unit struct (i.e. without any fields)
struct AlwaysEqual;


fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Using struct update syntax
    let mut user2 = User {
        email: String::from("another@example.com"),
        //username: String::from("anotherusername"),
        ..user1
    };

    user2.username = String::from("Chacha");

    // Doesn't work since user1.username was moved
    //println!("{}", user1.username); 
    println!("{}", user2.username);

    using_tuple_structs();
    let _subject = AlwaysEqual;
}
