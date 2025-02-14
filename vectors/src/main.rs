fn main() {

    // Empty vector
    let v: Vec<i32> = Vec::new();
    // vector with initial values
    let v = vec![1, 2, 3];
    // Empty mutable values
    let mut v: Vec<i32> = Vec::new();

    v.push(5);

    // Reading
    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There's no third element.")
    }
    if let Some(third) = v.get(2) {
        println!("If let got third number of {third}");
    }
    
    // Not exists:
    let v = vec![1, 2, 3, 4, 5];
    //let does_not_exist = &v[100]; // Will panic
    let does_not_exist = v.get(100);

    // Iterating
    let v = vec![100, 32, 57];
    for n_ref in &v {
        // n_ref has type &i32
        let n_times_two: i32 = 2 * *n_ref;
        println!("{n_times_two}");
    }

    let mut v = vec![100, 32, 57];
    for n_ref in &mut v {
        // n_ref has type &mut i32
        *n_ref += 50;
    }
    println!("{:03?}", v);

    // Safely using iterators. This is a desugared for loop
    use std::slice::Iter;  
    let mut v: Vec<i32>         = vec![1, 2];
    let mut iter: Iter<'_, i32> = v.iter();
    let n1: &i32                = iter.next().unwrap();
    let n2: &i32                = iter.next().unwrap();
    let end: Option<&i32>       = iter.next();

    //  Iterating using a range
    use std::ops::Range; 
    let mut v: Vec<i32>        = vec![1, 2];
    let mut iter: Range<usize> = 0 .. v.len();
    let i1: usize              = iter.next().unwrap();
    let n1: &i32               = &v[i1];

    // Using enum to hold different types in same vec
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


}
