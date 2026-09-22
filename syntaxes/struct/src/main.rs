fn main() {
    println!("结构体");
    let u = User {
        username: String::from("Name"),
        email: String::from("test@mail.com"),
        age: 9,
    };
    println!("{}, {}, {}", u.username, u.email, u.age);
    let u2 = User {
        username: String::from("Name2"),
        email: u.email,
        age: 10,
    };
    println!("{:#?}", u2);
    u2.print();
    dbg!(u2);
    let cr = User::create(String::from("User::create"));
    cr.print();
    dbg!(cr);

    // 元组定义风格
    // 元组结构体相当于给元组带上具体名称，提升类型安全性
    // struct RGB(i32, i32, i32);
    // struct Info(i32, i32, i32);
    println!("元组定义风格");
    #[derive(Debug)]
    struct Point(i32, i32, f64);
    let p = Point(12, 12, 15.8);
    println!("{:?}, {}-{}-{}", p, p.0, p.1, p.2);
    dbg!(p);

    let c = Color(255, 128, 0);
    println!("{:?}, {}-{}-{}", c, c.0, c.1, c.2);
    dbg!(c);

    let subject = AlwaysEqual;
    let s = dbg!(subject);
    s.print(5);
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age: u8,
}

impl User {
    fn print(&self) {
        println!("User: {}, {}, {}", self.username, self.email, self.age + 1);
    }

    fn create(username: String) -> User {
        User {
            username,
            email: String::from(""),
            age: 1,
        }
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;

impl AlwaysEqual {
    fn print(&self, count: i32) {
        println!("AlwaysEqual struct {count}");
    }
}