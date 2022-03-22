use std::borrow::BorrowMut;
use std::cell::Cell;
use std::io::Write;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::ring_buffer::RingBuffer;
use crate::SignalKind::{ROLLER, WHEEL};

pub enum SignalKind {
    WHEEL(Instant),
    ROLLER(Instant)
}

struct CurrentValues {
    kg_ha : f32,
    sum_meters : f32,
    sum_kg : f32
}

pub struct SignalProcessor {
    time_window     : Duration,
    current         : CurrentValues,
    signals_wheel   : Arc<Mutex<RingBuffer>>,
    signals_roller  : Arc<Mutex<RingBuffer>>
}
const wheel_meter : usize = 50;
const wheel_signals : usize =  417;
const SIGNALS_PER_METER : f32 = (wheel_signals as f32 ) / (wheel_meter as f32);

fn calculate_current_kilo_per_ha(signals_wheel : usize, signals_roller : usize) -> Option<f32> {

    let meters_in_timespan = signals_wheel as f32  / SIGNALS_PER_METER;
    let kilos_in_timespan  = signals_roller as f32 / _currentSignalsPerKilo;

    if meters_in_timespan > 0.0
    {
        let kilos_per_meter = kilos_in_timespan / meters_in_timespan;
        Some(kilos_per_meter * METERS_PER_HEKTAR)
    }
    else
    {
        None
    }
}

impl SignalProcessor {

    pub fn start_calculate(&mut self) -> JoinHandle<()> {

        let buf_wheel = self.signals_wheel.clone();
        let buf_roller = self.signals_roller.clone();

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));

            let now = Instant::now();

            let signals_timewindow_wheel = buf_wheel.lock().unwrap().signals_within_duration(&now, &self.time_window);
            let signals_timewindow_roller = buf_wheel.lock().unwrap().signals_within_duration(&now, &self.time_window);
            let kilos_per_ha = calculate_current_kilo_per_ha(signals_timewindow_wheel, signals_timewindow_roller);



        })
    }

    pub fn start_receive_signals(&mut self, signals_rx: Receiver<SignalKind>) -> JoinHandle<()> {

        let mut buf_wheel = self.signals_wheel.clone();
        let mut buf_roller = self.signals_roller.clone();

        thread::spawn( move || {
            for signal in signals_rx.iter() {
                match signal {
                    WHEEL(instant) => {
                        buf_wheel.lock().unwrap().push(instant);
                        print!("W");
                    },
                    ROLLER(instant) => {
                        buf_roller.lock().unwrap( ).push(instant);
                        print!(" R ");
                    }
                }
                std::io::stdout().flush();
            }
            println!("SignalProcessor ended");
        })
    }
    pub fn new(time_window : Duration) -> SignalProcessor {
        SignalProcessor {
            time_window,
            current: CurrentValues {
                kg_ha: 0.0,
                sum_meters: 0.0,
                sum_kg: 0.0
            },
            signals_wheel: Arc::new(Mutex::new(RingBuffer::new(2048))),
            signals_roller: Arc::new(Mutex::new(RingBuffer::new(2048)))
        }
    }
}
