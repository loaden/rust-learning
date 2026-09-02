#[derive(PartialEq, PartialOrd, Debug)]
pub struct Rectangle {
    pub length: u32,
    pub width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    pub fn test_should_panic(&self) {
        if self.width == 0 || self.length == 0 {
            panic!("Now it should panic.");
        }
    }
}