fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 1;
        println!("{}", i);
    }
    println!("{}", v.first().unwrap());

    let g = v.get(10);
    match g {
        Some(i) => println!("{}", i),
        None => println!("Not exist"),
    }

    let mut m = vec![Msg::Title(String::from("title")), Msg::Length(12)];

    m.push(Msg::Title(String::from("hello")));
    m.push(Msg::Length(88));

    for i in &mut m {
        match i {
            Msg::Title(x) => println!("String: {}", x),
            Msg::Length(x) => println!("i32: {}", x),
        }

        if let Msg::Title(t) = i {
            println!("if let Title: {}", t);
        } else if let Msg::Length(t) = i {
            println!("else if let Length: {}", t);
        }
    }

    let mut v = vec![1, 2, 3];
    v.push(4);
    let p = &v[3];
    println!("{}", p);
}

enum Msg {
    Title(String),
    Length(i32),
}
