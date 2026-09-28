fn main() {
    // 枚举用于描述固定数量选项的数据，如性别、大洲、星期、民族、月份等
    // 语法：
    // enum name {
    //     item1,
    //     item2,
    //     item3,
    //     ...
    // }
    #[allow(unused)]
    enum Continents {
        Asia,
        Africa,
        NorthAmerica,
        SouthAmerica,
        Antarctica,
        Europe,
        Oceania,
    }
    // 枚举常和match出现，基于模式匹配，可以很好的处理各种枚举值的情况。
    // 而枚举本身又限制了用户可输入的数据范围，不存在要求用户传入一个洲，却传入了一个国家的情况。
    fn get_hemisphere(continent: Continents) -> String {
        match continent {
            Continents::Asia => "主要位于东半球和北半球".to_string(),
            Continents::Africa => "地跨南北半球，主要位于东半球".to_string(),
            Continents::NorthAmerica => "主要位于西半球和北半球".to_string(),
            Continents::SouthAmerica => "主要位于西半球和南半球".to_string(),
            Continents::Antarctica => "位于南半球，跨所有经度".to_string(),
            Continents::Europe => "主要位于东半球和北半球".to_string(),
            Continents::Oceania => "主要位于南半球和东半球".to_string(),
        }
    }
    let c = Continents::NorthAmerica;
    println!("{}", get_hemisphere(c));

    // 类 C 枚举体
    // Rust 允许枚举变体设置整型数值，主要用于和C语言交互，或者在网络传输中节省空间
    #[repr(u16)]
    #[allow(unused)]
    enum HttpStatus {
        Ok = 200,
        NotFound = 404,
    }

    // 带参枚举体
    // 枚举的变体中，还允许携带参数。
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    fn print_ip_address(ip: IpAddr) {
        match ip {
            IpAddr::V4(a, b, c, d) => {
                println!("IPv4 地址: {}.{}.{}.{}", a, b, c, d);
            }
            IpAddr::V6(s) => {
                println!("IPv6 地址: {}", s);
            }
        }
    }
    print_ip_address(IpAddr::V4(10, 35, 96, 1));
    print_ip_address(IpAddr::V6("::1".to_string()));
    // 处理像这种类似于元组的传参，还可以使用类似于结构体的传参
    #[allow(unused)]
    enum Event {
        Click,
        KeyPress(char),
        Resize { width: u32, height: u32 },
    }
    let e = Event::Resize {
        width: 800,
        height: 600,
    };

    match e {
        Event::Click => println!("Clicked!"),
        Event::KeyPress(c) => println!("Key pressed: {}", c),
        Event::Resize { width, height } => {
            println!("Resized to {}x{}", width, height);
        }
    }

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
