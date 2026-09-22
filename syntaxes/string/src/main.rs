fn main() {
    // 集合字符串
    println!("集合字符串");
    let s1 = "Hello".to_string();
    let s2 = String::from("World");
    println!("{}, {}", s1, s2);

    let s3 = s1 + &s2; // s1 被移动，不能再使用
    println!("s3 = {}; s2 = {}", s3, s2);

    let mut s4 = "*S4* ".to_string();
    s4 += &s2;
    println!("s4 = {}, s2 = {}", s4, s2);

    let s5 = format!("{} + {}", s2, s4);
    println!("s5 = {}, s2 = {}", s5, s2);

    let s6 = "你好Rust".to_string();
    for c in s6.chars() {
        println!("for c in s6.chars = {}", c);
    }
    for b in s6.bytes() {
        println!("for b in s6.bytes = {}", b);
    }
}
