fn main() {
    // &T 不可变借用
    // 不可变借用不会获取所有权，原始变量仍然有效
    // 语法：let 借用: &类型 = &变量;
    println!("&T 不可变借用");
    let s: String = String::from("hello");
    let r: &String = &s;
    println!("原始字符串: {}", s);
    println!("借用内容: {}", r);

    // 尝试通过不可变借用修改数据会导致编译错误
    // r.push_str(", world"); // 编译错误！

    // 多个不可变借用可以同时存在
    // 多个不可变借用是安全的，因为它们都只是读取数据，不会造成数据竞争。读取操作本身是线程安全的。
    println!("多个不可变借用可以同时存在");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("{}, {}, {}", r1, r2, r3);

    // &mut T 可变借用
    // 可变借用允许你通过借用修改数据
    // 语法：let 借用: &mut 类型 = &mut 变量;
    println!("&mut T 可变借用");
    let mut s = String::from("hello");
    let r: &mut String = &mut s;
    // 可变借用使用期间，独占所有权
    // println!("{}", s); // 编译错误！
    r.push_str(", world");
    println!("修改后: {}", r);
    // 修改后可以正常编译
    // 由于之后不再使用r，所以 r 的生命周期已经结束
    println!("可变借用完成修改后可以正常访问原始变量");
    println!("{}", s);

    // 规则1：同一时间只能有一个可变借用 (防止了数据竞争)
    // 规则2：可变借用与不可变借用不能同时存在 (防止不可变借用的数据被意外修改)
    // 规则3：借用的生命周期不能超过其借用的数据 (借用必须始终指向有效的内存)
    // let r;
    // {
    //     let x = 5;
    //     r = &x; // 编译错误：x 的生命周期不够长
    // }

    // Rust 2018引入了更智能的借用检查Non-Lexical Lifetimes (NLL)，能够分析借用的实际使用范围
    // 编译器能够分析出r1和r2在println!之后不再使用，所以允许后续创建可变借用，提高了代码的灵活性
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 和 r2 在这里不再被使用，生命周期结束
    let r3 = &mut s; // 现在可以创建可变借用
    println!("{}", r3);

    // 不可变借用生命周期内，可变变量失去写权限
    println!("不可变借用生命周期内，可变变量失去写权限");
    let mut s: String = String::from("hello");
    let r: &String = &s;
    // s.push_str(", world"); // 编译报错
    println!("{}", r);
    s.push_str(", world"); // 编译通过，因为之后没使用r，r的生命周期已经结束

    // 可变借用生命周期内，可变变量同时失去读和写权限
    println!("可变借用生命周期内，可变变量同时失去读和写权限");
    let mut s: String = String::from("hello");
    let r: &mut String = &mut s;
    // s.push_str(", world"); // 编译报错，原变量失去写权限
    // println!("{}", s); // 编译错误，原变量失去读权限
    r.push_str(", world");
    println!("{}", r);
    s.push_str(", world"); // 编译通过，因为之后没使用r，r的生命周期已经结束
    println!("{}", s);

    // 解借用操作符*获取借用指向的实际值
    println!("解借用操作符*获取借用指向的实际值");
    let x = 5;
    let y = &x;
    println!("x = {}", x);
    println!("*y = {}", *y);
    // assert_eq!(5, y); // 编译错误：不能比较i32和&i32
    assert_eq!(5, *y); // 正确：解借用后比较

    // 多级借用
    let x = 5;
    let r1 = &x; // r1: &i32
    let r2 = &r1; // r2: &&i32
    let r3 = &r2; // r3: &&&i32
    assert_eq!(x, *r1);
    assert_eq!(x, **r2);
    assert_eq!(x, ***r3);

    // 自动解借用
    // 在使用.调用方法时，Rust会自动解借用
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &r1;
    let r3 = &r2;
    println!("直接调用: {}", s.len());
    println!("一级借用: {}", r1.len()); // 等价于 (*r1).len()
    println!("二级借用: {}", r2.len()); // 等价于 (**r2).len()
    println!("三级借用: {}", r3.len()); // 等价于 (***r3).len()

    // obj.function()：编译器自动处理所有层级的解借用
    // function(*obj)：需要手动解借用确保参数类型匹配
    let x: i32 = 5;
    let r1 = &x;
    let r2 = &r1;
    // 方法调用：自动解借用
    let _ = x.abs(); // 直接调用
    let _ = r1.abs(); // 自动解借用
    let _ = r2.abs(); // 自动多重解借用
    // 函数调用：需要手动解借用
    let _ = i32::abs(x); // 传值
    let _ = i32::abs(*r1); // 手动解借用
    let _ = i32::abs(**r2); // 手动多重解借用
}
