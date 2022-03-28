use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::ring_buffer::RingBuffer;

pub fn start(signals_wheel : Arc<Mutex<RingBuffer>>, signals_roller : Arc<Mutex<RingBuffer>>) {
    thread::spawn(move || {
        const SCALE_ROLLER: usize = 20;

        let mut i : usize = 1;
        loop {
            thread::sleep(Duration::from_millis(50));
            let now = Instant::now();
            signals_wheel.lock().unwrap().push(now);
            if i == SCALE_ROLLER {
                signals_roller.lock().unwrap().push(now);
                i = 0;
            }

            i += 1;
        }
    });
    println!("FAKE signals started");
}
