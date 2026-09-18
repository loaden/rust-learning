fn main() {
    let a = Foo { x: 8, ..Default::default() };
    println!("{:?}", a);
}
#[derive(Debug)]
struct Foo {
    size: (u32, u32),
    x: i32,
    y: i32,
    visiable: bool,
}
impl Default for Foo {
    fn default() -> Self {
        Self {
            size: (1920, 1080),
            x: 0,
            y: 0,
            visiable: true,
        }
    }
}