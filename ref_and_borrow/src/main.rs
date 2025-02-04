fn main() {
    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut vec[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", vec);
}