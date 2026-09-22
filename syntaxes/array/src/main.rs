fn main() {
    // 数组是Rust内建的原始集合类型，它表示同类型、长度固定、有序的元素集合。
    // 数组的类型为[T; N]，其中T表示内部的类型，N表示数组的长度。
    // 为了保证其编译期可以确定大小的特性，Rust要求N必须在编译期可以求值
    // 语法：
    // let arr: [T; N] = [val1, val2, val3...];
    // let arr: [T; N] = [val; N];
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

    // 三种最常见的编译期可以确定值的表达式：字面量、const 常量、CFTE 函数。
    const fn const_func() -> usize {
        10
    }
    const LEN: usize = 10;
    let arr = [2026; 10];
    println!("let arr = [2026; 10] {:?}", arr);
    let arr = [2026; LEN];
    println!("let arr = [2026; LEN] {:?}", arr);
    let arr = [2026; const_func()];
    println!("let arr = [2026; const_func()] {:?}", arr);
}
