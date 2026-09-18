use self::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

pub fn rc_refcell_loop_test() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count: {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count aftr b creation: {}", Rc::strong_count(&a));
    println!("b initial rc count: {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after loop: {}", Rc::strong_count(&b));
    println!("a rc count after loop: {}", Rc::strong_count(&a));

    // 引起循环引用，造成栈溢出
    // println!("a next item = {:?}", a.tail());

    // 打破环引用
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::new(Nil);
        println!("b rc count after break: {}", Rc::strong_count(&b));
        println!("a rc count after break: {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());
    }
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
