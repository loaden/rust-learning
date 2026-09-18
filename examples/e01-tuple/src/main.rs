use std::os::unix::net::UnixStream;

fn main() {
    let mut t = (10, "string", 2.3);
    let (a, b, c) = t;
    println!("{},{},{}", a, b, c);
    t.1 = "test";
    println!("{},{},{}", t.0, t.1, t.2);
    // 可以跳过某些值，只要在模式中使用下划线 _ 占位即可
    let (d, _, f) = t;
    println!("{},{}", d, f);
    // 只有一个元素的元组必须加一个逗号，否则它会被当作普通的括号表达式
    let t = (10,);
    println!("{:?}", t);
    // 元组可以作为函数的返回值
    let result = tuple_func();
    println!("{:?}", result);
    // 不带任何值的元组有一个特殊名字，叫做 单元（unit）。这种值以及其对应的类型都写作 ()，表示空值或空的返回类型
    // 如果一个表达式没有返回任何其他值，它就会隐式返回单元值
    let unit = ();
    println!("{:?}", unit);
}

fn tuple_func() -> (&'static str,) {
    match rand::random::<u32>() % 5 + 1 {
        1 => ("one",),
        2 => ("two",),
        3 => ("three",),
        4 => ("four",),
        5 => ("five",),
        _ => ("unknown",),
    }
}
