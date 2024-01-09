#[allow(unused_imports)]
pub use mutex::Mutex;
#[allow(unused_imports)]
pub use rwlock::RwLock;

mod mutex;
mod rwlock;
pub type MutexNoIrq<T> = Mutex<T>;
