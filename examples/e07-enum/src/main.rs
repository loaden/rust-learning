fn main() {
    let q = Message::Quit;
    let p = Message::Point { x: 32, y: 66 };
    let t = Message::Title(String::from("title"));
    let c = Message::Color(255, 0, 0);
    print_enum(&q);
    print_enum(&p);
    print_enum(&t);
    print_enum(&c);
    q.print();
    p.print();
    t.print();
    c.print();

    match p {
        Message::Point { x, y } => println!("Message::Point {}, {}", x, y),
        _ => (),
    }

    if let Message::Point { x: 32, y: 66 } = p {
        println!("Message::Point {:?}", p);
    }

    if let Message::Point { x, y } = p {
        println!("Message::Point {} - {}", x, y);
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Point { x: i32, y: i32 },
    Title(String),
    Color(i32, i32, i32),
}

fn print_enum(m: &Message) -> &Message {
    println!("{:?}", m);
    m
}

impl Message {
    fn print(&self) {
        println!("{:?}", self);
    }
}
