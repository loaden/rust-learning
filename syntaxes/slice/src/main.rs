fn main() {
    let mut s = String::from("hello");
    println!("{}", &s[1..]);
    println!("{}", test_slice(&s));

    // s.clear();
    let r = string_slice(&s);
    println!("main: {}", r);

    let r = string_slice_mut(&mut s);
    // s.clear();
    println!("main: {}", r);

    s.clear();

    let i = [1, 2, 3, 4, 5];
    let a = array_slice(&i);
    for j in a {
        println!("main: {}", j);
    }

    let str = "Hello World!";
    println!("main: {}", str_slice(&&&&&&&&&&&&&&&&&&str));
}

fn str_slice(s: &str) -> &str {
    println!("str_slice: {}", &s[1..]);
    &s[..2]
}

fn string_slice_mut(s: &mut String) -> &str {
    *s += " world";
    println!("string_slice_mut: {}", s);
    &s[1..]
}

fn string_slice(s: &String) -> &str {
    println!("string_slice: {}", *s);
    &s[2..4]
}

fn test_slice(s: &String) -> String {
    s[..3].to_string()
}

fn array_slice(a: &[i32]) -> &[i32] {
    for i in &a[1..4] {
        println!("array_slice: {}", i);
    }
    &a[..2]
}
