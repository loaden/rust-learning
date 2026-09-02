pub fn function_pointer() {
    let r = test(add, 10);
    println!("{}", r);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter().map(|i| i.to_string()).collect();
    println!("{:?}", list_of_numbers);
    println!("{:?}", list_of_strings);

    let list_of_numbers = vec![1, 2, 3, 4, 5];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter().map(ToString::to_string).collect();
    println!("{:?}", list_of_numbers);
    println!("{:?}", list_of_strings);

    let list_of_statuses: Vec<Status> = (0_u32..20)
        .map(Status::Value).collect();
    println!("构造器风格 {:?}", list_of_statuses);
    let list_of_statuses: Vec<Status> = (0_u32..20)
        .map(|i| Status::Value(i)).collect();
    println!("闭包风格 {:?}", list_of_statuses);

    let f = return_closure();
    println!("return closure: {}", (*f)(1));
}

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1 )
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn add(x: i32) -> i32 {
    x + 1
}

fn test(f: fn(i32) -> i32, i: i32) -> i32 {
    f(i) + f(i)
}
