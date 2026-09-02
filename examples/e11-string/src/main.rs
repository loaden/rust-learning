fn main() {
    let s1 = "Hello, world!".to_string();
    let s2 = String::from("-s2");
    println!("{}, {}", s1, s2);

    let s3 = s1 + &s2;
    println!("{}", s3);

    let mut s4 = "hi".to_string();
    s4 += &s2;
    println!("{}", s4);

    let s5 = format!("{} = {}", s2, s4);
    println!("{}", s5);

    let s6 = "中文1英a".to_string();
    for c in s6.chars() {
        println!("{}", c);
    }
    for b in s6.bytes() {
        println!("{}", b);
    }
}
