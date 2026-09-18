fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let v: Vec<_> = v.iter().map(|i| i + 1).collect();
    println!("{:?}", v);
    let v: Vec<_> = v.iter().filter(|i| *i % 2 == 0).collect();
    println!("{:?}", v);
}
