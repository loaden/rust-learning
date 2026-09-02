use std::fmt;
use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub struct Millimeters(pub u32);
pub struct Meters(pub u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}

pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
}

pub struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot::fly for Human");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard::fly for Human");
    }
}

impl Human {
    pub fn fly(&self) {
        println!("Human::fly");
    }
}

pub trait Animal {
    fn name() -> String;
}

pub struct Dog;
impl Dog {
    pub fn name() -> String { String::from("Dog") }
}

impl Animal for Dog {
    fn name() -> String {
        String::from("Puppy")
    }
}

pub trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        println!("outline print {}", self.to_string());
    }
}

// pub struct Point {
//     x: i32,
//     y: i32,
// }

impl OutlinePrint for Point {
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'({}-{})'", self.x, self.y)
    }
}