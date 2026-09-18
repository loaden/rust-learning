pub mod mixed {
    use self::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;

    pub fn rc_refcell_pointer_test() {
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
        *value.borrow_mut() += 10;
        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }

    #[derive(Debug)]
    enum List<T> {
        Cons(Rc<RefCell<T>>, Rc<List<T>>),
        Nil,
    }
}
