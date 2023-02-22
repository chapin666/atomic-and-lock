pub mod channel_v1;
pub mod channel_v2;
pub mod channel_v3;
pub mod channel_v4;
pub mod channel_v5;

#[cfg(test)]
mod tests {
    use super::channel_v1::Channel;
    use super::channel_v2::Channel as OneshotChannel;
    use super::channel_v3::channel;
    use super::channel_v4::Channel as AvoidallocChannel;
    use super::channel_v5::Channel as BlockChannel;
    use std::process;
    use std::thread;
    use std::time::Duration;

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
