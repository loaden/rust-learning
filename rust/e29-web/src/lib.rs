use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// 创建线程池
    ///
    /// 线程池中线程的数量
    /// # Panics
    ///
    /// `new`函数会在size为0时触发panic。
    ///
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {
            // 创建线程并将它们存储到动态数组
        }

        ThreadPool {
            threads
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {}
}