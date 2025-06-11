unsafe fn dangerous() {}

// Calling external functions using Foreign Function Interface (FFI)
// The "C" part is the application binary interface (ABI), see list of supported ones: https://doc.rust-lang.org/reference/items/external-blocks.html#abi
unsafe extern "C" {
    // Everything is unsafe except if explicity using safe keyword
    safe fn abs(input: i32) -> i32;
}


static mut COUNTER: u32 = 0;

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}


unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}


fn main() {
    
    // Creating immutable and mutable raw pointers
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    // Raw pointers an only be derefrenced in an unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Unsafe functions must be called within unsafe blocks
    unsafe {
        dangerous();
    }

    // Calling abs from C
    println!("Absolute value of -3 according to C: {}", abs(-3));
    
    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
    
}

/*
When writing unsafe code, you might want to check that what you have written actually
 is safe and correct. One of the best ways to do that is to use Miri, an official Rust
 tool for detecting undefined behavior. Whereas the borrow checker is a static tool
 that works at compile time, Miri is a dynamic tool that works at runtime. It checks
 your code by running your program, or its test suite, and detecting when you violate
 the rules it understands about how Rust should work.
*/

