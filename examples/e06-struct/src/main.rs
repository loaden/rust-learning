fn main() {
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

    // 元组定义风格
    #[derive(Debug)]
    struct Point(i32, i32, f64);
    let p = Point(12, 12, 15.8);
    println!("{:?}", p);

    let cr = User::create(String::from("username"));
    println!("{:#?}", cr);

    let c = Color(255, 128, 0);
    println!("{:#?}", c);
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