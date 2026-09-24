fn main() {
    let m = Message::quit();
    if m.is_quit() {
        m.process();
    }
    let m = Message::mov(10, 20);
    m.process();
    let m = Message::write("text".to_string());
    m.process();
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

impl Message {
    // 关联函数创建不同变体
    fn quit() -> Self {
        Message::Quit
    }

    fn mov(x: i32, y: i32) -> Self {
        Message::Move { x, y }
    }

    fn write(text: String) -> Self {
        Message::Write(text)
    }

    // 实例方法处理消息
    fn process(&self) {
        match self {
            Message::Quit => println!("退出程序"),
            Message::Move { x, y } => println!("移动到位置: ({}, {})", x, y),
            Message::Write(text) => println!("写入消息: {}", text),
        }
    }

    // 检查变体类型
    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }
}