mod mutex;
mod atomic_reference;
fn main() {
    println!("Hello, world!");
    mutex::mutex_one_thread_can_access();
    atomic_reference::atomic_reference_count();
}
