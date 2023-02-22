pub mod rwlock_v1;
pub mod rwlock_v2;
pub mod rwlock_v3;

#[cfg(test)]
mod tests {

    use super::rwlock_v3::RwLock;

    #[test]
    fn test_rwlock_v3() {
        let lock = RwLock::new(5);
        {
            let r1 = lock.read();
            let r2 = lock.read();
            assert_eq!(*r1, 5);
            assert_eq!(*r2, 5);
        }
        {
            let mut w = lock.write();
            *w += 1;
            assert_eq!(*w, 6);
        }
    }
}
