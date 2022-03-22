use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use crate::SignalKind;

pub fn start(signals_tx : Sender<SignalKind>) -> JoinHandle<()> {
    thread::spawn(move || {
        const scaleRoller : usize = 20;

        println!("starting FAKE signals");
        let mut i : usize = 1;
        loop {
            thread::sleep(Duration::from_millis(50));
            let now = Instant::now();
            signals_tx.send( SignalKind::WHEEL(now) );
            if i == scaleRoller {
                signals_tx.send( SignalKind::ROLLER(now) );
                i = 0;
            }

            i += 1;
        }
    })
}
