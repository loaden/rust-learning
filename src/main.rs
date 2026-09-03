fn main() {
    // 数组定义
    println!("数组定义");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    let b = [0; 5]; // 定义一个包含5个0的数组
    println!("{:?}", b);

    // 表达式
    println!("表达式");
    let x = {
        let y = 1;
        y + 1
    };
    println!("{}", x);
    let x = if x > 1 { 1 } else { 0 };
    println!("{}", x);
}
