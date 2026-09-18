use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn thread_channel() {
    let (tx, rx) = mpsc::channel();

    let h = thread::spawn(move || {
        let msg = "hi".to_string();
        thread::sleep(Duration::from_secs(1));
        tx.send(msg).unwrap();
        // println!("send msg: {}", msg);
        thread::sleep(Duration::from_secs(2));

        let vals = vec![1, 2, 3, 4, 5];
        for v in vals {
            tx.send(v.to_string()).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        drop(tx);
        thread::sleep(Duration::from_secs(3));
    });

    println!("begin receiving");
    let msg = rx.recv().unwrap();
    println!("end receiving");
    println!("Got: {}", msg);

    for received in rx {
        println!("Got: {}", received);
    }

    println!("sending done");
    h.join().unwrap();
    println!("main done");
}