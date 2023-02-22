pub mod mutex_v1;

#[cfg(test)]
mod tests {
    use super::mutex_v1::Mutex;
    use std::thread;
    use std::time::Instant;

    #[test]
    fn test_basic_mutex() {
        let m = Mutex::new(0);
        std::hint::black_box(&m);
        let start = Instant::now();
        for _ in 0..5_000_000 {
            *m.lock() += 1;
        }
        let duration = start.elapsed();
        println!("locked {} times in {:?}", *m.lock(), duration);
    }

    #[test]
    fn test_lock_contended() {
        let m = Mutex::new(0);
        std::hint::black_box(&m);
        let start = Instant::now();
        thread::scope(|s| {
            for _ in 0..4 {
                s.spawn(|| {
                    for _ in 0..5_000_000 {
                        *m.lock() += 1;
                    }
                });
            }
        });
        let duration = start.elapsed();
        println!("locked {} times in {:?}", *m.lock(), duration);
    }
}
