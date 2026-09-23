fn main() {
    // 借用是“对数据的引用”，而切片（slice）是一种更精细粒度的引用
    // 它不是借整个值，而是借用值里连续的一部分元素。
    // 切片可以理解为“借用的子集”。
    // 如果 &T 借的是整个房子，那 &T[start..end] 借的只是其中几间房。
    // Rust 的切片主要有两种：
    //  * 数组切片（&[T]）
    //  * 字符串切片（&str）
    // 切片始终是左闭右开区间，即 [start, end) 不包含

    // 数组切片
    // 任何数组或 Vec<T> 都可以通过 [..] 语法创建切片
    println!("任何数组或 Vec<T> 都可以通过 [..] 语法创建切片");
    let arr = [10, 20, 30, 40, 50];
    let part = &arr[1..4]; // 取索引 1, 2, 3，类型为 &[i32] 的切片引用
    println!("切片内容: {:?}", part); // [20, 30, 40]

    // 切片的索引方式
    let r = ..3;
    let part = &arr[r];
    println!("..3切片内容: {:?}", part);
    let r = 2..;
    let part = &arr[r];
    println!("2..切片内容: {:?}", part);
    let r = ..;
    let part = &arr[r];
    println!("..切片内容: {:?}", part);

    // 可变切片可以修改原数组的一部分数据
    // 切片会继承借用规则：可变切片独占访问，不可与其他借用共存
    let mut arr = [1, 2, 3, 4, 5];
    let slice = &mut arr[2..4];
    slice[0] *= 10;
    slice[1] *= 10;
    println!("修改后数组: {:?}", arr); // [1, 2, 30, 40, 5]

    // 将一个数组拆分为两个可变切片
    let mut arr = [1, 2, 3, 4, 5];
    let (b1, b2) = arr.split_at_mut(2); // 以索引2为界拆分
    b1[0] = 10;
    b2[0] = 20;
    println!("双可变借用修改后数组: {:?}", arr); // [10, 2, 20, 4, 5]

    // 字符串切片
    println!("字符串切片");
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("切片1: {}", hello);
    println!("切片2: {}", world);
    let s = "hello";
    assert_eq!(s, hello);

    // 在底层实现上，切片是一个胖指针，它同时保存了
    //  * 1. 一个指向起始位置的指针
    //  * 2. 一个长度信息
    let arr = [10, 20, 30];
    let s = &arr[..2];
    println!("切片长度: {}", s.len()); // 3
}
