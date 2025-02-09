fn area_args(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function without &self (i.e. not methods). Often used for constructors.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

// Checking whether a separate impl block works. It does.
impl Rectangle {
    fn height(&self) -> bool {
        self.height > 0
    }
}

fn multiple_args_method() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_args(width1, height1)
    );

    let rect1 = (40, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    let rect1 = Rectangle {
        width: 50,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    dbg!(rect1.width());
    dbg!(rect1.height());
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);

    multiple_args_method();

    let square = Rectangle::square(5);

    // These two lines are equivalent
    let area1 = square.area();
    let area2 = Rectangle::area(&square);
    assert_eq!(area1, area2);

    // Box on heap
    let r = &mut Box::new(Rectangle {
        width: 1,
        height: 2,
    });
    // Rust automatically adds references/dereferences to make things match. So these are equivalent
    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    assert_eq!(area1, area2);
}
