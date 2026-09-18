use std::thread;
use std::time::Duration;

pub fn thread_move() {
    let v = vec![1, 2, 3];
    let h = thread::spawn(move || {
        for i in &v {
            thread::sleep(Duration::from_secs(1));
            println!("thread: {}", i);
        }
        println!("{:?}", v);
    });

    h.join().unwrap();
}
