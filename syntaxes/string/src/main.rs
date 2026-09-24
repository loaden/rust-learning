fn main() {
    // Rust的字符串分为两种，原生字符串str和集合字符串String。
    // 不论哪一种，内部存储的都是utf8编码序列，也就是说每个字符的大小是不定的。
    // 例如"你好Rust"这个字符串
    // '你'和'好'两个分别占用了三个字节，而后续的"Rust"分别占用一字节。
    // 存放 utf‑8 的字节序列 `e4 bd a0 e5 a5 bd 52 75 73 74`
    let s = String::from("你好Rust");
    println!(
        "{}, 栈上大小={}, 字符串有效字节长度={}",
        s,
        size_of_val(&s),
        s.len()
    );
    println!("字符数量={}", s.chars().count());
    // 遍历打印
    for c in s.chars() {
        print!("{}, ", c);
    }
    println!();
    for b in s.bytes() {
        print!("{:02x}, ", b);
    }
    println!();
    // 在String中，存储三部分内容：
    // - ptr：指向堆区的指针
    // - len：字符串的长度
    // - capacity：堆区目前已分配的容量
    // 一开始capacity为8，后面每次扩容都会把当前的capacity * 2，一百个字符最后只需要四次扩容，减少了堆区内存的申请次数。
    let mut s = String::new();
    for _ in 0..100 {
        s.push('x');
        if s.len() == s.capacity() {
            println!("len: {}, cap: {}", s.len(), s.capacity());
        }
    }
}
