fn main() {
    let mut t = (10, "string", 2.3);
    let (a, b, c) = t;
    println!("{},{},{}", a, b, c);
    t.1 = "test";
    println!("{},{},{}", t.0, t.1, t.2)
}
