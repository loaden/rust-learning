fn main() {
    let s1 = String::from("value");
    let s2 = s1;
    println!("{}", takes_ownership(s2));
    // println!("{}", s1);
    // println!("{}", s2);
    let s3 = String::from("hello");
    let mut s4 = takes_ownership_gives_back(s3);
    s4.push_str("string");
    // println!("{}", s3);
    println!("{}", s4);
}

fn takes_ownership(mut s: String) ->usize {
    s.push('A');
    println!("takes_ownership: {}", s);
    s.capacity()
}

fn takes_ownership_gives_back(mut s: String) -> String {
    s.push('B');
    println!("takes_ownership_gives_back: {}", s);
    s
}
