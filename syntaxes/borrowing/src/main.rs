fn main() {
    let mut s = String::from("Hello, world!");
    s.push_str("string");
    let n = test_borrowing(&s);
    println!("{}", n);
    let (ss, n) = test_borrowing_mut(&mut s);
    // ss.push_str("string");
    println!("{}, {}, {}", s, ss, n);
    let _sr1 = &mut s;
    let sr2 = &mut s;
    // _sr1.push('A');
    sr2.push('B');

    let mut tx = 5;
    let ty = &mut tx;
    // tx += 3; // use of borrowed `tx`
    *ty += 2;
    tx += 3;
    println!("{}", tx);
}

fn test_borrowing(s: &String) -> usize {
    // s.push('@');
    s.capacity()
}

fn test_borrowing_mut(s: &mut String) -> (String, usize) {
    s.push('@');
    let mut sf = String::from("test_borrowing");
    sf.push('A');
    (sf, s.len())
}