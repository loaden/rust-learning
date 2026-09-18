fn main() {
    let mut a: Option<&str> = Some("Hello, world");
    if let Some(a) = a {
        println!("{}", a);
    }
    a = None;
    match a {
        Some(s) => println!("The string is {}", s),
        None => println!("The string is None"),
    }

    let b: Result<u32, _> = "25".parse();
    match b {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", e),
    }

    let v = vec![1, 2, 3, 4, 5, 6, 7];
    for ( i, v ) in v.iter().enumerate() {
        println!("{}: {}", i, v);
    }
    println!("{}", v.len());

    let (x, y, z) = (10, 20, 30);
    println!("x = {}, y = {}, z = {}", x, y, z);

    let (x, y, _) = (4, 5, 6);
    println!("x = {}, y = {}, z = {}", x, y, z);

    let (x, ..) = (7, 8, 9, 10);
    println!("x = {}, y = {}, z = {}", x, y, z);

    let mut tx = 5;
    let ty = &mut tx;
    *ty += 2;
    tx += 3;

    match tx {
        1..=10 => println!("{} is between 1 and 10", tx),
        _ => (),
    }

    let p = Point { x: 0, y: 20 };
    let Point { x, y } = p;
    let Point { x: a, y: b } = p;
    // let Point { a, b } = p;
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("At ({}, {})", x, y),
    }

    let msg = Message::Quit;
    match msg {
        Message::ChangeColor(r, ..) => println!("{}", r),
        Message::Move { x, y } => println!("xy:{},{}", x, y),
        Message::Write(s) => println!("s = {}", s),
        Message::Quit => println!("Quit"),
    }

    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("case 1 = {}", x),
        _ => (),
    }

    let mut cfg = None;
    let value = Some(5);
    match (cfg, value) {
        (Some(_), Some(_)) => (),
        _ => {
            cfg = value;
        }
    }
    println!("cfg: {:?}", cfg);
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}