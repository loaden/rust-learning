use std::cell::RefCell;
use std::cell::Ref;
pub fn test_return_str() {
    let d = Data { data: RefCell::new(String::from("hello")) };
    println!("{}, {}", d.get1(), d.get2());
    let r = d.get2();
}
struct Data { data: RefCell<String>, }
impl Data {
    fn get1(&self) -> String {
        self.data.borrow().to_string()
    }
    fn get2(&self) -> Ref<'_, String> {
        self.data.borrow()
    }
}