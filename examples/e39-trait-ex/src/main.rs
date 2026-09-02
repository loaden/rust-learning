fn main() {
    slider(0..=10, 5, StepMessage::SliderChanged);
    slider(0..=10, 5, |_| StepMessage::SliderChanged(8));
    let b = <Demo as Sandbox>::new();
    b.test();
    b.update();
}

#[derive(Clone)]
enum StepMessage {
    SliderChanged(u8),
}

#[allow(unused_variables)]
fn slider<T, Message>(
    range: std::ops::RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message,
) where
    T: Copy + From<u8> + std::cmp::PartialOrd,
    Message: Clone,
{
}

trait Application: Sized {
    fn new() -> (Self, String);
    fn title(&self) -> String;
    fn test(&self);
}

trait Sandbox {
    fn new() -> Self;
    fn title(&self) -> String;
}

impl<T> Application for T
where
    T: Sandbox,
{
    fn new() -> (Self, String) {
        (T::new(), "".to_string())
    }
    fn title(&self) -> String {
        T::title(self)
    }
    fn test(&self) {
        println!("fn test(&self)");
    }
}

struct Demo;
impl Sandbox for Demo {
    fn new() -> Self {
        Demo
    }
    fn title(&self) -> String {
        "Demo".to_string()
    }
}

impl Demo {
    fn update(&self) {
        println!("fn update(&self)");
    }
}
