use std::thread;
use std::time::Duration;

pub fn thread_join() {

    let h = thread::spawn(|| {
        for i in 1..=8 {
            thread::sleep(Duration::from_secs(1));
            println!("thread: {}", i);
        }
    });

    for i in 10..15 {
        thread::sleep(Duration::from_secs(1));
        println!("main: {}", i);
    }

    h.join().unwrap();
}
