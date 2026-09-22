use std::f32::consts::E;

fn main() {
    // 枚举用于描述固定数量选项的数据，如性别、大洲、星期、民族、月份等
    // Rust 允许枚举变体设置整型数值，主要用于和C语言交互，或者在网络传输中节省空间
    // Rust 允许枚举变体设置结构体，主要用于描述复杂数据结构
    let q = Message::Quit;
    let p = Message::Point { x: 32, y: 66 };
    let t = Message::Title(String::from("title"));
    let c = Message::Color(255, 0, 0);
    q.print();
    p.print();
    t.print();
    c.print();

    match p {
        Message::Point { x, y } => {
            println!("Message::Point {{ x, y }}  x={},y={}", x, y)
        }
        _ => (),
    }

    if let Message::Point { x: 32, y: 66 } = p {
        println!("if let Message::Point {{ x: 32, y: 66 }} = p");
    }

    if let Message::Point { x, y } = p {
        println!("if let Message::Point {{ x, y }} = p  x={},y={}", x, y);
    }

    if let Message::Title(s) = t {
        println!("if let Message::Title(s) = t  s={}", s);
    }

    if let Message::Color(r, g, b) = c {
        println!(
            "if let Message::Color(r, g, b) = c  r={},g={},b={}",
            r, g, b
        );
    }
    // 事件枚举示例
    println!("事件枚举示例");
    enum Event {
        Click,
        KeyPress(char),
        Resize { width: u32, height: u32 },
    }
    let e = Event::Resize {
        width: 800,
        height: 600,
    };
    match_event(e);
    fn match_event(e: Event) {
        match e {
            Event::Click => println!("Clicked!"),
            Event::KeyPress(c) => println!("Key pressed: {}", c),
            Event::Resize { width, height } => {
                println!("Resized to {}x{}", width, height);
            }
        }
    }
    let e = Event::KeyPress('a');
    match_event(e);
    let e = Event::Click;
    match_event(e);
}

enum Message {
    Quit,
    Point { x: i32, y: i32 }, // 命名字段，类似结构体
    Title(String),
    Color(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        match self {
            Message::Quit => println!("Message::Quit"),
            Message::Point { x, y } => {
                println!("Message::Point {{ x, y }}  {},{}", x, y)
            }
            Message::Title(s) => println!("Message::Title(s)  {}", s),
            Message::Color(r, g, b) => {
                println!("Message::Color(r, g, b)  {},{},{}", r, g, b)
            }
        }
    }
}
