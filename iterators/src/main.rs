fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }


    // map creates a new iterator
    let v1: Vec<i32> = vec![1, 2, 3];
    v1.iter().map(|x| x + 1); // Causes warning: lazy and must be used.

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);


    /* From quiz question
        v.iter() produces an Iterator<Item = &i32>. The .filter() call takes 
        an Iterator<Item = T> as input, and passes &T to its predicate.
        Therefore x: &&i32 for a

        By contrast on line 4, when .map() takes an Iterator<Item = T> as input,
        it passes T to its closure. Therefore the closure in map takes &i32 as input.
        The multiplication operator * is implemented for &i32, so x does not need to
        be dereferenced in x * 2. The operation x * 2 produces a value of type i32,
        so the result of the map is an Iterator<Item = i32>. The filter then 
        takes x : &i32, which also does not need a dereference to do x % 2.
        Now you know!
    */
    let v = vec![1, 2, 3, 4];
    let a: Vec<_> = v.iter().filter(|x: &&i32| *x % 2 == 0).map(|x: &i32| x * 2).collect();
    let b: Vec<_> = v.iter().map(|x: &i32| x * 2).filter(|x: &i32| x % 2 == 0).collect();
    println!("{} {}", a[0], b[0]);


}
