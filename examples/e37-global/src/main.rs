use std::ops::AddAssign;
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;

static APP: OnceLock<App> = OnceLock::new();
// static mut APP2: OnceLock<App2> = OnceLock::new();

use lazy_static::lazy_static;
lazy_static! {
    static ref APP3: Mutex<App3> = Mutex::new(App3::new());
}

// use once_cell::sync::Lazy;
// static mut APP4: Lazy<Mutex<App4>> = Lazy::new(|| Mutex::new(App4::new()));

fn main() {
    // 子线程中调用
    let handle = thread::spawn(|| {
        let logger = App::global();
        logger.log("thread message".to_string());
    });

    // 主线程调用
    let logger = App::global();
    logger.log("some message".to_string());

    // App2::global();
    // unsafe {
    //     let logger2 = APP2.get_mut().unwrap();
    //     logger2.save("app2_".to_string());
    //     logger2.save("app2_".to_string());
    // }

    {
        let mut logger3 = APP3.lock().unwrap();
        logger3.save("app3_".to_string());
    }

    let handle2 = thread::spawn(|| {
        let mut log = APP3.lock().unwrap();
        log.save("app3_".to_string());
        thread::sleep(Duration::from_secs(3));
    });

    thread::sleep(Duration::from_secs(1));
    let mut logger3 = APP3.lock().unwrap();
    logger3.save("app3_".to_string());

    // unsafe {
    //     let mut logger4 = APP4.lock().unwrap();
    //     logger4.save("app4_".to_string());
    // }

    handle.join().unwrap();
    handle2.join().unwrap();
}

#[derive(Default)]
struct App {
    name: String,
}

impl App {
    fn global() -> &'static App {
        // 获取或初始化
        APP.get_or_init(|| {
            println!("App is being created..."); // 初始化打印
            App::default()
        })
    }

    fn log(&self, message: String) {
        println!("{}: {}", self.name, message);
    }
}

// #[derive(Default)]
// struct App2 {
//     msgs: String,
// }

// impl App2 {
//     fn global() -> &'static App2 {
//         unsafe {
//             // 获取或初始化
//             APP2.get_or_init(|| {
//                 println!("App2 is being created..."); // 初始化打印
//                 App2::default()
//             })
//         }
//     }

//     fn save(&mut self, message: String) {
//         self.msgs.add_assign(message.as_str());
//         println!("self.msgs: {}", self.msgs);
//     }
// }

#[derive(Default)]
struct App3 {
    msgs: String,
}

impl App3 {
    fn new() -> App3 {
        println!("App3 is being created..."); // 初始化打印
        App3::default()
    }

    fn save(&mut self, message: String) {
        self.msgs.add_assign(message.as_str());
        println!("App3 self.msgs: {}", self.msgs);
    }
}

// #[derive(Default)]
// struct App4 {
//     msgs: String,
// }

// impl App4 {
//     fn new() -> App4 {
//         println!("App4 is being created..."); // 初始化打印
//         App4::default()
//     }

//     fn save(&mut self, message: String) {
//         self.msgs.add_assign(message.as_str());
//         println!("App4 self.msgs: {}", self.msgs);
//     }
// }
