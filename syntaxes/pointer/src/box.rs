use self::List::{Cons, Nil};
use std::ops::Deref;
use std::fmt::Display;

pub fn box_pointer_test() {
    let r1: List<u32> = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{:#?}", r1);
    let r2: List<f64> = Cons(1.12, Box::new(Cons(2.23, Box::new(Nil))));
    println!("{:#?}", r2);

    let r3 = MyBox::new(String::from("hello"));
    println!("{}", *r3);
    hello(&r3);
    drop(r3);

    let r4 = SmartPointer::new(String::from("smart"));
    println!("{}", *r4);
}

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("drop MyBox");
    }
}

struct SmartPointer<T: Display> {
    x: T,
}
impl<T: Display> SmartPointer<T> {
    fn new(x: T) -> SmartPointer<T> {
        SmartPointer { x }
    }
}
impl<T: Display> Deref for SmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.x
    }
}
impl<T: Display> Drop for SmartPointer<T> {
    fn drop(&mut self) {
        println!("drop SmartPointer: {}", &self.x);
    }
}

fn hello(s: &str) {
    println!("hello call: {}", s);
}