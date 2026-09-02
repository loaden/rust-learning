pub fn dynamically_sized_type() {
    let b: Box<str> = "bbb".to_owned().into_boxed_str();
    println!("{:?}, {:?}", b, b.as_ref());
}