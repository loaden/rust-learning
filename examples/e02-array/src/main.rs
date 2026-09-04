fn main() {
    let a = [1, 2, 3, 4, 5, 6];
    println!("{}", a.len());
    let mut a = ["ok"; 5];
    a[0] = "zero";
    a.sort();
    println!("{}, {}", a[0], a.last().unwrap());
    println!("for e in a");
    for e in a {
        println!("{:?}", e);
    }
    println!("for e in a.iter()");
    for e in a.iter() {
        println!("{:?}", e);
    }
    println!("for (i, e) in a.iter().enumerate()");
    for (i, e) in a.iter().enumerate() {
        println!("{:?}: {:?}", i, e);
    }
}
