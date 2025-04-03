struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // c.drop(); // Not possible. It's implicitly called when going out of scope
    // For early dropping, use drop(c) instead of c.drop()
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before end of main.");
}
