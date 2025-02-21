fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    let my_team = "Foo";
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(my_team.to_string(), 100);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    let score = scores.get(my_team)
                        .copied()
                        .unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hash maps and ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("{}", field_name);


    // Overwriting value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // overwrites
    scores.entry(String::from("Blue")).or_insert(8); // only overwrites if it doesn't exist
    let red_value = scores.entry(String::from("Red")).or_insert(1); // only overwrites if it doesn't exist
    *red_value += 1;

    println!("{scores:?}");


    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");



}
