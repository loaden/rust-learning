mod r#trait;
use crate::r#trait::{Point, Human, Pilot, Wizard, Dog, Animal, OutlinePrint};
use crate::r#trait::{Millimeters, Meters};

mod wrapper;
use crate::wrapper::wrapper_test;

mod dst;
use crate::dst::dynamically_sized_type;

mod function_pointer;
use crate::function_pointer::function_pointer;

use std::io::Result;
// type Result<T> = std::io::Result<T>;

fn main() -> Result<()> {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    println!("{:?}", p3);
    p3.outline_print();

    let mm = Millimeters(1);
    let m = Meters(1);
    let nm = mm + m;
    println!("{}", nm.0);

    let h = Human;
    h.fly();
    Pilot::fly(&h);
    Wizard::fly(&h);

    println!("{}", Dog::name());
    println!("{}", <Dog as Animal>::name());

    wrapper_test();

    dynamically_sized_type();
    function_pointer();

    Ok(())
}
