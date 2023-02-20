pub mod avoidalloc_channel;
pub mod block_channel;
pub mod channel;
pub mod oneshot_channel;
pub mod safe_channel;
pub mod spin_lock;

#[cfg(test)]
mod tests {
    use crate::avoidalloc_channel::Channel as AvoidallocChannel;
    use crate::block_channel::Channel as BlockChannel;
    use crate::channel::Channel;
    use crate::oneshot_channel::Channel as OneshotChannel;
    use crate::safe_channel::channel;
    use crate::spin_lock::SpinLock;
    use std::process;
    use std::thread;
    use std::time::Duration;

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

    #[test]
    fn test_channel() {
        let channel = Channel::new();
        thread::scope(|s| {
            s.spawn(|| {
                for i in 0..100 {
                    channel.send(i);
                    thread::sleep(Duration::from_micros(50));
                }
            });

            s.spawn(|| loop {
                let num = channel.receive();
                println!("{}", num);
                if num == 99 {
                    process::exit(0);
                }
            });
        });
    }

    #[test]
    fn test_oneshot_channel() {
        let channel = OneshotChannel::new();
        let t = thread::current();

        thread::scope(|s| {
            s.spawn(|| {
                channel.send("hello world!");
                channel.send("hello world2!");
                t.unpark();
            });
            while !channel.is_ready() {
                thread::park();
            }
            assert_eq!(channel.receive(), "hello world!");
        });
    }

    #[test]
    fn test_safe_channel() {
        thread::scope(|s| {
            let (sender, receiver) = channel();
            let t = thread::current();
            s.spawn(move || {
                sender.send("hello world!");
                t.unpark();
            });

            while !receiver.is_ready() {
                thread::park();
            }
            assert_eq!(receiver.receive(), "hello world!");
        });
    }

    #[test]
    fn test_avoidalloc_channel() {
        let mut channel = AvoidallocChannel::new();
        thread::scope(|s| {
            let (sender, receiver) = channel.split();
            let t = thread::current();

            s.spawn(move || {
                sender.send("hello world!");
                t.unpark();
            });

            while !receiver.is_ready() {
                thread::park();
            }

            assert_eq!(receiver.receive(), "hello world!");
        });
    }

    #[test]
    fn test_block_channel() {
        let mut channel = BlockChannel::new();
        thread::scope(|s| {
            let (sender, receiver) = channel.split();

            s.spawn(move || {
                sender.send("hello world!");
            });

            assert_eq!(receiver.receive(), "hello world!");
        });
    }
}
