fn main() {
    // 数组是Rust内建的原始集合类型，它表示同类型、长度固定、有序的元素集合。
    // 数组的类型为[T; N]，其中T表示内部的类型，N表示数组的长度。
    // 数组有两种定义形式：
    // 第一种形式确定所有元素，每个val都必须是T类型的元素，而且必须有N个val。
    // 第二种形式把所有元素都初始化为同一个值val，比如let arr = ["hello", 10]就是定义了一个十个hello的数组。
    // 语法：
    // let arr: [T; N] = [val1, val2, val3...];
    // let arr: [T; N] = [val; N];
    // Rust的原生数组通常定义在栈上，基于T和N在编译期就可以确定内存占用情况。
    // 为了保证其编译期可以确定大小的特性，Rust要求N必须在编译期可以求值。
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    let arr = ["hello"; 10];
    println!("{:?}", arr);

    // 三种最常见的编译期可以确定值的表达式：字面量、const 常量、CFTE 函数。
    const fn const_func() -> usize {
        10
    }
    const LEN: usize = 10;
    let arr = [2026; 10];
    println!("字面量: {:?}", arr);
    let arr = [2026; LEN];
    println!("const 常量: {:?}", arr);
    let arr = [2026; const_func()];
    println!("CFTE 函数: {:?}", arr);
    // 下标访问
    // 语法： let val = arr[pos];
    let mut arr = [26; 10];
    arr[1] = 2026;
    // arr[10] = 2027; // index out of bounds: the length is 10 but the index is 10
    println!("{:?}", arr);
    // 遍历
    let arr = [1, 2, 3];
    for i in arr {
        print!("{},", i);
    }
    println!();
    for i in arr.iter() {
        print!("{},", i);
    }
    println!();
    for (i, e) in arr.iter().enumerate() {
        println!("index={}, value={}", i, e);
    }
}
