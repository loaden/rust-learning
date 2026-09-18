use std::{cmp::PartialOrd, fmt::Display};

fn main() {
    let a = 32;
    let b = 35;
    println!("{}", bigger_test(a, b));
    let c = String::from("cccc");
    let d = String::from("dddd");
    println!("{}", bigger_test2(&c, &d));
    println!("{}", bigger_test(c, d));

    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 1.0, y: 3.5 };
    println!("{},{},{},{}", p1.x, p1.y, p2.x, p2.y);
}

fn bigger_test<T: PartialOrd + Display>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

fn bigger_test2<'a, T: PartialOrd>(x: &'a T, y: &'a T) -> &'a T {
    if x > y {
        x
    } else {
        y
    }
}

struct Point<T> {
    x: T,
    y: T,
}
