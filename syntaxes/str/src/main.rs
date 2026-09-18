fn main() {
    // 原生字符串str
    println!("原生字符串str: {}", r#"Hello, "Rust"!"#);
    // str 是动态大小类型 DST(Dynamically‑Sized Type),本身不能直接放在栈上，只能借助指针形式使用（&str、Box<str>）
    // 不能直接持有 str；必须包裹在指针 / 智能指针外层，形成「指针 + 长度」的胖指针 (fat‑pointer)
    // DST：编译时无法知道确切的大小
    // 规则：DST 类型本身不能直接作为变量、栈上分配；不能 let s: str = "abc";（编译报错！）
    // &str：引用胖指针，底层：(*const u8, usize)，一个指针指向字节起始，附带长度字段
    // str 本质就是特殊化的 [u8]，约束必须合法 UTF‑8
    // str本体 DST；必须套一层指针（引用 / Box）得到胖指针；String是拥有所有权的字符串容器 (Sized)
    let s: &str = "Hello, Rust!";
    println!("字符串s: {}", s);
    let s: Box<str> = Box::from("Hello, Rust!");
    println!("Box<str> s: {}", s);
}
