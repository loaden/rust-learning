fn main() {
    let a1 = [1, 2, 3, 4, 5, 6];
    let mut a2 = ["ok"; 5];
    a2[0] = "zero";
    a2.sort();
    println!("{}", a1.len());
    println!("{}, {}", a2[0], a2.last().unwrap());
    for e in a2 {
        println!("for {}", e);
    }
}
