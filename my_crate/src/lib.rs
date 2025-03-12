//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.


/// Adds one to the number given
///
/// # Example
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: u64) -> u64 {
    x + 1
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Adds the string "world" to `s`.
///
/// # Example
/// ```
/// let mut s = String::new();
/// my_crate::add_world(&mut s);
/// assert_eq!(s, "Hello world");
/// ```
pub fn add_world(s: &mut String) {
    s.push_str("world");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn test_add_world() {
        let mut s = String::new();
        add_world(&mut s);
        assert_eq!(s, "world");
    }
}
