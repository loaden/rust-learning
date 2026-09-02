mod join;
mod r#move;
mod channel;
mod mpsc;
mod mutex;

fn main() {
    // crate::join::thread_join();
    // crate::r#move::thread_move();
    // crate::channel::thread_channel();
    // crate::mpsc::thread_channel_mpsc();
    crate::mutex::thread_mutex();
    crate::mutex::thread_mutex_arc();
}
