// 在Rust中，假设把一个struct看成一个类，属性一般放在结构体中，而方法就放在它的impl和trait中
// impl不是struct专属的特性
// impl可以用于struct、enum、union以及trait object
fn main() {
    let mut p = Person::new("zhangsan".to_string(), 18);
    if p.is_adult() {
        p.say_hello();
        Person::say_hello(&p); // 等效于 p.say_hello();
    }
    p.have_birthday();
    println!("{}", p.info());

    // 链式调用
    Text::new("hello world!".to_string()).bold().italic().show();

    // 方法结束后销毁
    // 当方法执行完后对象就不再需要时，使用self可以明确表示消费该对象
    let ticket = Ticket::new("邓紫棋".to_string(), "张三".to_string());
    ticket.read_singer().read_user();
    ticket.use_ticket(); // 消费ticket
    // ticket.use_ticket(); // error[E0382]: use of moved value: `ticket`

    // self 允许四种情况
    // - &self 不可变引用
    // - &mut self 可变引用
    // - self / mut self 获得所有权
    // - 指向Self的智能指针
    let d = Data::new(2026);
    println!("{}", d.boxed().process_in_heap());

    // 关联常量
    // 关联常量是定义在impl块中的常量值，它们与类型本身相关联，而不是与具体的实例相关联。
    // 面向对象中的静态方法和静态成员在Rust中分别叫做关联函数和关联常量。
    // 关联一词，一方面是有意避开了面向对象风格的命名，另一方面，它表示这个函数或者常量，只是和当前的类型有关联
    // 比如PI这个常量与圆Circle强相关，涉及到面积体积之类的计算。
    // 实际上，Rust是一个多范式语言，它同时吸收了面向对象，函数式编程等多种思想，并且把它们做合适的裁剪
    // 比如Rust没有所谓的类型继承，甚至没有类这个概念，而是组合优先。
    // Rust是一个多范式语言，而不是一个纯面向对象语言。
    let c = Circle { radius: 2.0 };
    println!("面积 = {}, 周长 = {}, PI = {}, {}", c.area(), c.circumference(), Circle::get_pi(), Circle::PI);
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    // 创建一个新Person
    // 在impl内部，Self特指类型本身，此处等于Person
    // 这种第一个参数不是self的方法，称为关联函数，在一些面向对象语言中也叫做静态方法。
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    // 简单的介绍方法
    // 第一个参数是&self。这种写法相当于self: &Self，注意区分首字母大小写，首字母小写的是参数，大写的是类型
    // 对于这种方法，必须通过.func()的形式调用
    // 通过.操作符，会把.前面的变量作为第一个参数self传进方法中
    // 第一个参数是&self的函数也叫做实例方法
    fn say_hello(&self) {
        println!("你好，我是{}", self.name);
    }
}

// 第二个impl块：年龄相关方法
// 一个类型可以有多个impl块，这在组织代码时非常有用。只要方法名不冲突，你可以将相关的方法分组到不同的impl块中
// 甚至可以把impl放到不同文件中。
impl Person {
    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    // &mut self 可变引用
    // 当以&mut self 接收时，相当于self: &mut Self按照可变引用形式接收参数。
    // 此时内部的self会拿到一个可变借用，但是不占据所有权。
    // 这种情况下你可以在方法内部修改self的值，说明这个方法会对数据进行某些改动。
    fn have_birthday(&mut self) {
        self.age += 1;
        println!("过生日！现在{}岁", self.age);
    }
}

// 第三个impl块：工具方法
impl Person {
    fn info(&self) -> String {
        format!("姓名: {}, 年龄: {}", self.name, self.age)
    }
}

struct Text {
    content: String,
    is_bold: bool,
    is_italic: bool,
}

impl Text {
    fn new(content: String) -> Self {
        Text {
            content,
            is_bold: false,
            is_italic: false,
        }
    }

    // 每个方法都消费self并返回新的self
    fn bold(mut self) -> Self {
        self.is_bold = true;
        self
    }

    fn italic(mut self) -> Self {
        self.is_italic = true;
        self
    }

    fn show(&self) {
        let bold = if self.is_bold { "粗体" } else { "正常" };
        let italic = if self.is_italic { "斜体" } else { "正常" };
        println!("内容: {} [{}, {}]", self.content, bold, italic);
    }
}

struct Ticket {
    singer: String,
    user: String,
}

impl Ticket {
    fn new(singer: String, user: String) -> Self {
        Ticket { singer, user }
    }

    // 使用门票，消费后门票作废
    fn use_ticket(self) {
        println!("{} 已使用 {} 的门票", self.user, self.singer)
    }

    // 查看门票信息
    fn read_singer(&self) -> &Self {
        println!("这是 {} 演唱会的门票", self.singer);
        self
    }

    fn read_user(&self) -> &Self {
        println!("使用者为: {}", self.user);
        self
    }
}

struct Data {
    value: i32,
}

impl Data {
    fn new(value: i32) -> Self {
        Data { value }
    }

    fn process_in_heap(self: Box<Self>) -> i32 {
        println!("处理堆上的数据: {}", self.value);
        self.value * 2
    }

    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

struct Circle {
    radius: f64,
}

impl Circle {
    // 关联常量是定义在impl块中的常量值，它们与类型本身相关联，而不是与具体的实例相关联。
    // 同一类型的所有实例，访问到关联常量都是同一份数据，关联常量在内存中只会存储一份。
    // 关联常量其实相当于其它面向对象语言中的静态成员。
    // 关联常量使用const进行定义：
    const PI: f64 = 3.14;

    // 在方法中使用关联常量
    fn area(&self) -> f64 {
        Self::PI * self.radius * self.radius
    }

    fn circumference(&self) -> f64 {
        2.0 * Self::PI * self.radius
    }

    // 在关联函数中也可以访问关联常量
    fn get_pi() -> f64 {
        Self::PI
    }
}
