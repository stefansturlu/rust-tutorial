fn main() {
    let mut s = String::new();

    // String literal, stored in program's binary
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    // This is equivalent
    let s = String::from("initial contents");


    // UTF-8 endoded examples
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("Góðan dæinn!");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Growing strings
    let mut s = String::from("foo");
    s.push_str("bar");
    s += "baz";
    println!("{}", s);

    // push_str takes &str to avoid ownership
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // Use push for characters
    let mut s = String::from("lo");
    s.push('l');

    // Combining with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // Does not take ownership of any inputs
    let s = s1 + "-" + &s2 + "-" + &s3; // Takes ownership of s1 so it can't be used

    // Indexing into strings is not supported due to how strings are stored in memory.
    let s1 = String::from("hello");
    // let h = s1[0]; // Will throw error!

    // String is a wrapper over Vec<u8>
    let hello = String::from("Hola"); // len: 4
    let hello = String::from("Здравствуйте"); // len: 24. Not 12, because of UTF-8
    let s = &hello[0..4]; // must be a char boundary, otherwise we get a runtime error

    for c in "Зд".chars() {
        println!("{c}");
    }
    for c in "नमस्ते".chars() {
        // Getting grapheme clusters from strings is not provided by the standard library
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }





}
