mod oop;
use oop::AveragedCollection;

mod gui;
use gui::{Screen, Button, SelectBox};

mod state;
use state::Post;

mod state_ex;
mod question;
mod rust;

fn main() {
    let mut a = AveragedCollection::new();
    a.add(3);
    a.add(7);
    a.add(2);
    match a.remove(3) {
        Ok(v) => println!("Value: {:?}", v),
        Err(e) => println!("Error: {:?}", e),
    }
    println!("Average: {}", a.average());

    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 100,
                height: 100,
                label: "&Save".to_string(),
            }),
            Box::new(SelectBox {
                width: 100,
                height: 100,
                options: vec!["Yes".to_string(), "No".to_string()],
            }),
        ],
    };

    screen.run();

    let mut p = Post::new();
    p.add_text("text");
    println!("Drawt content: {}", p.content());
    p.request_review();
    println!("PendingReview content: {}", p.content());
    p.approve();
    println!("Approved content: {}", p.content());

    let mut p = state_ex::Post::new();
    p.add_text("state-ex");
    println!("Drawt content: {}", p.content());
    p.request_review();
    println!("PendingReview content: {}", p.content());
    p.reject();
    println!("Drawt content: {}", p.content());
    p.approve();
    println!("Approved content: {}", p.content());
    p.request_review();
    p.approve();
    println!("Approved content: {}", p.content());

    crate::question::test_return_str();

    let mut post = crate::rust::Post::new();
    post.add_text("text");
    post.add_text("ok");
    let post2 = post.request_review();
    // post.request_review(); // `post` moved due to this method call
    let post3 = post2.approve();
    // post2.approve(); // `post2` moved due to this method call
    println!("Content: {}", post3.content());
}
