use std::sync::mpsc::{channel, Receiver};
use std::time::{Duration, Instant};

use crate::ring_buffer::RingBuffer;
use crate::SignalKind::{ROLLER, WHEEL};

pub enum SignalKind {
    WHEEL,
    ROLLER
}

struct CurrentValues {
    kg_ha : f32,
    sum_meters : f32,
    sum_kg : f32
}

pub struct SignalProcessor {
    signal_rx: Receiver<SignalKind>,
    time_window : Duration,
    current : CurrentValues,
    signals_wheel : RingBuffer,
    signal_roller : RingBuffer
}

impl SignalProcessor {
    pub fn process(&mut self) {
        for signal in self.signal_rx.iter() {
            match signal {
                WHEEL=> self.signals_wheel.push(timestamp),
                ROLLER => self.signal_roller.push(timestamp)
            }
        }
    }
    pub fn new(signal_rx: Receiver<SignalKind>, time_window : Duration) -> SignalProcessor {
        SignalProcessor {
            signal_rx,
            time_window,
            current: CurrentValues {
                kg_ha: 0.0,
                sum_meters: 0.0,
                sum_kg: 0.0
            },
            signals_wheel: RingBuffer::new(2048),
            signal_roller: RingBuffer::new(2048)
        }
    }
}
