use atomic_wait::{wait, wake_all, wake_one};
use mutex::mutex_v1::MutexGuard;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

pub struct Condvar {
    counter: AtomicU32,
}

impl Condvar {
    pub const fn new() -> Self {
        Self {
            counter: AtomicU32::new(0),
        }
    }

    pub fn notify_one(&self) {
        self.counter.fetch_add(1, Relaxed);
        wake_one(&self.counter);
    }

    pub fn notify_all(&self) {
        self.counter.fetch_add(1, Relaxed);
        wake_all(&self.counter);
    }

    pub fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> MutexGuard<'a, T> {
        let counter_value = self.counter.load(Relaxed);
        let mutex = guard.mutex;
        drop(guard);
        wait(&self.counter, counter_value);
        mutex.lock()
    }
}
