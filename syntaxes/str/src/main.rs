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

    // [u8]：字节切片本体（DST 动态大小），不能直接变量；&[u8] 是字节切片的胖指针
    // str：字符串切片本体（DST）；&str 是字符串切片胖指针
    // 第 1 段：let s: &[u8] = b"Hello, Rust!";
    // b"xxx" → 字节字面量（byte literal） b"Hello, Rust!" 的类型是 &'static [u8]
    // 字符串字面量前面加b，代表这是字节串，存的就是原始 u8 字节，不做 UTF‑8 语义约束（可以存任意字节）
    // 生命周期是 'static：数据固化在程序的只读段，不在堆、不在栈。
    // s 的类型 &[u8]（字节切片）； {:?} debug 打印，输出每个字节的数字：
    let s: &[u8] = b"Hello, Rust!";
    println!("字节切片s: {:?}", s);
    // String::from(...)：在堆上分配内存，存放 UTF‑8 文本；s是拥有所有权的 String (Sized 结构体，栈存指针 + 长度 + 容量，数据堆上) 这里发生重影 (shadowing，变量重遮蔽，不是修改原来变量) [..5] 是切片索引语法；对 String 做索引切片取引用，得到 &str！
    // s[..5] 试图直接取本体 str (DST，不允许)；前面加&取引用 → &str（字符串切片胖指针）
    // ..5：下标从 0 开始，到下标5 之前停止（左闭右开） 0:H,1:e,2:l,3:l,4:o →截取前 5 个字符 "Hello"
    // 坑点： String 用[起始..结束]做切片的时候，是按字节索引！不是按字符！ 如果里面有中文，一旦切片刚好切到一个汉字 UTF‑8 字节中间，直接运行时 panic 崩溃！ 比如 "你好"，一个汉字占 3 字节，&s[0..2]直接 panic。
    let s = String::from("Hello, Rust!");
    let s = &s[..5];
    println!("字符串切片s: {}", s);
}
