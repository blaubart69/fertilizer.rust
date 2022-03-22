use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use crate::SignalKind;

fn start<F>(mut on_signal : F)
    where F: FnMut(SignalKind) + std::marker::Send + 'static {
    thread::spawn(move || {
        const scaleRoller : usize = 20;
        let mut i : usize = 1;
        loop {
            thread::sleep(Duration::from_millis(50));
            on_signal(SignalKind::WHEEL);

            if i == scaleRoller {
                on_signal(SignalKind::ROLLER);
                i = 0;
            }

            i += 1;
        }
    });
}
