pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Box<dyn Draw> is a trait object, i.e. a stand-in
    // for any type inside a Box that implements Draw
    pub components: Vec<Box<dyn Draw>>,
}
/*
 Note: We could have used generic type params, i.e. Screen<T: Draw>, but a
 generic type parameter can only be substituted with one concrete type at a
 time, whereas trait objects allow for multiple concrete types to fill in
 for the trait object at runtime.
 */
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
