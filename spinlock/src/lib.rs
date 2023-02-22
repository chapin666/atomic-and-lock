pub mod spinlock_v1;

#[cfg(test)]
mod tests {
    use super::spinlock_v1::SpinLock;
    use std::thread;

    #[test]
    fn test_spin_lock() {
        let x = SpinLock::new(Vec::new());
        thread::scope(|s| {
            s.spawn(|| x.lock().push(1));
            s.spawn(|| {
                let mut g = x.lock();
                g.push(2);
                g.push(2);
            });
        });
        let g = x.lock();
        assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);
    }
}
