fn main() {
    // 文档：cargo doc --open
    // 数组定义
    println!("数组定义");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    let b = [0; 5]; // 定义一个包含5个0的数组
    println!("{:?}", b);

    // 表达式
    println!("表达式");
    let x = {
        let y = 1;
        y + 1
    };
    println!("{}", x);
    let x = if x > 1 { 1 } else { 0 };
    println!("{}", x);

    // 循环遍历
    println!("循环遍历");
    for i in 0..5 {
        print!("{},", i);
    }
    println!();
    for i in a.iter() {
        print!("{},", i);
    }
    println!();
    for i in a.iter().rev() {
        print!("{},", i);
    }
    println!();

    // 所有权规则
    println!("所有权规则");
    /*
    Rust 中的每一个值都有一个所有者（owner）。
    在同一时间内，值有且仅有一个所有者。
    当所有者离开作用域，这个值将被丢弃。
    Rust 在变量离开作用域时会自动调用 drop 函数来释放资源。
    Rust 永远不会自动地创建深度拷贝（deep copy）。
    一旦某种类型实现了 Copy trait，那么它的值就会在赋值时被复制，而不是移动。
    Copy trait 只适用于那些在栈上分配数据的类型，比如整数类型、浮点数类型、布尔类型和字符类型。
    Copy trait 不适用于在堆上分配数据的类型，比如 String 和 Vec<T>。
    实现 Drop trait 的类型将不能实现 Copy trait。
    如果元组包含的类型都实现了 Copy trait，那么这个元组也会实现 Copy trait。
    将变量传递给函数将会触发移动（move）或拷贝（copy），具体取决于变量的类型。
    通过引用（reference）传递变量不会触发移动或拷贝，也叫做借用（borrowing）。
    对于特定作用域中的特定数据来说，Rust 允许存在唯一可变引用（mutable reference）或者任意数量的不可变引用（immutable reference），但不能同时存在可变引用和不可变引用。
    引用总是有效的，Rust 编译器会在编译时检查引用的有效性，确保引用不会悬空。
    */
    // 切片引用
    println!("切片引用");
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..4];
    println!("{:?}", slice);
    let slice = &a[1..=4];
    println!("{:?}", slice);
    let slice = &a[..4];
    println!("{:?}", slice);
    let slice = &a[2..];
    println!("{:?}", slice);
    let slice = &a[..];
    println!("{:?}", slice);
    // 字符串切片
    println!("字符串切片");
    let s = String::from("hello world");
    let slice: &str = &s[6..];
    println!("{:?}", slice);
    let s = "hello world";
    let slice: &str = &s[6..];
    println!("{:?}", slice);
}
