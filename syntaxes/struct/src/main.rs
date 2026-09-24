fn main() {
    // 结构体是最常见的将相关的数据字段组合在一起的类型，它有三种类型：
    // 具名结构体、元组结构体、单元结构体

    // 具名结构体
    // 具名结构体具名结构体就是指带有名字的结构体，语法如下：
    // struct name {
    //     item1: type1,
    //     item2: type2,
    //     item3: type3,
    //     ...
    // }

    // 通过struct关键字定义一个结构体，name是该结构体的名称
    // 一个结构体内可以有多个不同类型的元素，在{ }内使用逗号分隔，以item : type的形式定义。
    // 同样的，最后行的逗号可以保留也可以省略。
    struct Person {
        name: String, // 内部包含姓名和年龄字段。
        age: i32,
    }
    let p = Person {
        name: String::from("Alice"),
        age: 20,
    };
    // 访问结构体内部的变量，通过var.item的形式：
    println!("name: {}, age: {}", p.name, p.age);
    // 字段初始化简写
    let age = 18;
    let _ = Person {
        name: String::from("zhangsan"),
        age, // 可以直接把age: age简化为age
    };
    // 结构体更新语法
    // 如果你已经有一个结构体了，希望基于现有结构体创建一个新的结构体，并且只修改部分字段，就可以使用结构体更新语法。
    #[allow(unused)]
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let user1 = User {
        active: true,
        username: String::from("zhangsan"),
        email: String::from("zhangsan@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // value partially moved here
    };
    // println!("{:?} - {:?}", user1, user2); // value borrowed here after partial move
    println!("{:?}", user2);

    // 元组结构体
    // 元组结构体相当于给元组带上了具体的名称，语法如下：
    // struct name(type1, type2, type3...);
    // 通过struct定义一个元组结构体，在结构体名称后使用()定义每一个元素的类型，多个类型之间用逗号,分隔，最后一个逗号可以保留。
    // 除此之外，元组结构体尾部必须添加分号，而具名结构体不需要。
    // 元组定义风格
    // 元组结构体相当于给元组带上具体名称，提升类型安全性
    // struct RGB(i32, i32, i32);
    // struct Info(i32, i32, i32);
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    let c = Color(255, 128, 0);
    println!("{:?}, {}-{}-{}", c, c.0, c.1, c.2);
    dbg!(c);

    // 单元结构体
    // 当一个结构体内什么也没有，就称为单元结构体。
    struct People;
    let p1 = People;
    let p2 = People;
    // 在debug模式下，p1和p2的地址是不同的，因为它们确实是不同的变量，逻辑上在内存中占据不同的空间。
    // release模式下，由于这个单元结构体本身其实没有任何内容，它的大小实际为0，它的地址也没有什么实际意义。
    // 因此Rust会把全局所有的People都指向同一个实例，从而提高效率。
    println!("p1 addr: {:p}", &p1);
    println!("p2 addr: {:p}", &p2);
    // 具名结构体impl
    struct PrintCount;
    impl PrintCount {
        fn print(&self, count: i32) {
            println!("PrintCount = {count}");
        }
    }
    let p = PrintCount;
    p.print(2026);
}
