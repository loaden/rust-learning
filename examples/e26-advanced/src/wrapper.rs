use std::fmt;
use std::ops::Deref;
pub fn wrapper_test() {
    let mut vs = VecString(vec![
        String::from("hello"),
        String::from("world"),
    ]);
    vs.test_wrapper();
    println!("{}, {}", vs, vs.len());
}
struct VecString(Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'[{}]'", self.0.join(", "))
    }
}
impl Deref for VecString {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl VecString {
    fn test_wrapper(&mut self) {
        self.0.push(String::from("test wrapper"));
    }
}
