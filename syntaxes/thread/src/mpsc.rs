use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn thread_channel_mpsc() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![1, 2, 3, 4, 5];
        for v in vals {
            tx.send(v.to_string()).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![6, 7, 8, 9, 10];
        for v in vals {
            tx1.send(v.to_string()).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    println!("sending done");
}