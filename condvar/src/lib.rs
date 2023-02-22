pub mod condvar_v1;
pub mod condvar_v2;

#[cfg(test)]
mod tests {

    use super::condvar_v1::Condvar;
    use super::condvar_v2::Condvar as AvoidSyscallCondvar;
    use mutex::mutex_v1::Mutex;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_basic_condvar() {
        let mutex = Mutex::new(0);
        let condvar = Condvar::new();

        let mut wakeups = 0;

        thread::scope(|s| {
            s.spawn(|| {
                thread::sleep(Duration::from_secs(1));
                *mutex.lock() = 123;
                condvar.notify_one();
            });

            let mut m = mutex.lock();
            while *m < 100 {
                m = condvar.wait(m);
                wakeups += 1;
            }
            assert_eq!(*m, 123);
        });
        assert!(wakeups < 10);
    }

    #[test]
    fn test_avoid_syscall_condvar() {
        let mutex = Mutex::new(0);
        let condvar = AvoidSyscallCondvar::new();

        let mut wakeups = 0;

        thread::scope(|s| {
            s.spawn(|| {
                thread::sleep(Duration::from_secs(1));
                *mutex.lock() = 123;
                condvar.notify_one();
            });

            let mut m = mutex.lock();
            while *m < 100 {
                m = condvar.wait(m);
                wakeups += 1;
            }

            assert_eq!(*m, 123);
        });

        // Check that the main thread actually did wait (not busy-loop),
        // while still allowing for a few spurious wake ups.
        assert!(wakeups < 10);
    }
}
