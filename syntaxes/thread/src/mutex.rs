use std::thread;
use std::sync::{Mutex, Arc};

pub fn thread_mutex() {
    let m = Mutex::new(5);
    {
        println!("m: {:?}", m);
        let mut num = m.lock().unwrap();
        *num += 1;
        println!("m: {:?}", m);
        drop(num);
        println!("m: {:?}", m);
    }

    println!("m: {:?}", m);
}

pub fn thread_mutex_arc() {
    let m = Arc::new(Mutex::new(0));
    let mut h = vec![];

    for _ in 0..10 {
        let mm = Arc::clone(&m);
        let handle = thread::spawn(move || {
            let mut num = mm.lock().unwrap();
            *num += 1;
        });
        h.push(handle);
    }

    for handle in h {
        handle.join().unwrap();
    }

    println!("Result: {}", *m.lock().unwrap());
}