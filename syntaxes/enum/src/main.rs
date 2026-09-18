fn main() {
    let q = Message::Quit;
    let p = Message::Point { x: 32, y: 66 };
    let t = Message::Title(String::from("title"));
    let c = Message::Color(255, 0, 0);
    q.print();
    p.print();
    t.print();
    c.print();

    match p {
        Message::Point { x, y } => println!("Message::Point {{ x, y }}  x={},y={}", x, y),
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
        println!("if let Message::Color(r, g, b) = c  r={},g={},b={}", r, g, b);
    }
}

enum Message {
    Quit,
    Point { x: i32, y: i32 },
    Title(String),
    Color(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        match self {
            Message::Quit => println!("Message::Quit"),
            Message::Point { x, y } => println!("Message::Point {{ x, y }}  {},{}", x, y),
            Message::Title(s) => println!("Message::Title(s)  {}", s),
            Message::Color(r, g, b) => println!("Message::Color(r, g, b)  {},{},{}", r, g, b),
        }
    }
}
