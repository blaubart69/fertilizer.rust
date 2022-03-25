use std::io::Write;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

use crate::ring_buffer::RingBuffer;
use crate::SignalKind::{ROLLER, WHEEL};

pub enum SignalKind {
    WHEEL(Instant),
    ROLLER(Instant)
}

struct CurrentValues {
    kg_ha : f32,
    sum_meters : f32,
    sum_kg : f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Duenger {
    pub name: String,
    pub kg: f32
}

struct DuengerSettings {
    name : String,
    signals_per_kilo_duenger : f32
}

#[derive(Clone)]
pub struct SignalProcessor {
    time_window     : Duration,
    current         : Arc<Mutex<CurrentValues>>,
    settings        : Arc<Mutex<DuengerSettings>>,
    signals_wheel   : Arc<Mutex<RingBuffer>>,
    signals_roller  : Arc<Mutex<RingBuffer>>
}

pub const FILENAME_DUENGER_JSON : &'static str = "./conf/duenger.json";

//
// 10000 m² devediert durch 15m Breite des Düngerers
//
const METERS_PER_HEKTAR : f32 = 10000f32 / 15f32;
//
// signals per meter of the Duengergerät "Amazone"
//
const WHEEL_SIGNALS_FOR_50_METER: usize =  417;
const SIGNALS_PER_METER : f32 = (WHEEL_SIGNALS_FOR_50_METER as f32 ) / (50 as f32);
//
// Konstante für's "Abdrehen" - 30 Signale vom Zahnrad
//
const ROLLER_SIGNALS_ONE_ROTATION : u32 = 30;

fn calculate_current_kilo_per_ha(signals_wheel : usize, signals_roller : usize, signals_per_kilo_duenger : &f32) -> Option<f32> {

    if signals_wheel == 0 {
        None
    }
    else {
        let meters_in_timespan = signals_wheel as f32  / SIGNALS_PER_METER;
        let kilos_in_timespan  = signals_roller as f32 / signals_per_kilo_duenger;

        let kilos_per_meter = kilos_in_timespan / meters_in_timespan;
        let kilos_per_hektar = kilos_per_meter * METERS_PER_HEKTAR;

        Some(kilos_per_hektar)
    }
}

fn sum_meters_and_kilos(signals_wheel_last_calc : usize, signals_roller_last_calc : usize, signals_per_kilo_duenger : &f32) -> (f32,f32) {

    let meter_since_last_calc  = signals_wheel_last_calc as f32  / SIGNALS_PER_METER;
    let kilos_since_last_calc  = signals_roller_last_calc as f32 / signals_per_kilo_duenger;

    ( meter_since_last_calc, kilos_since_last_calc )
}

impl SignalProcessor {



    pub fn set_duenger(&self, name : &str, kilos : f32) {
        let mut guard = self.settings.lock().unwrap();
        guard.name = name.to_string();
        guard.signals_per_kilo_duenger = self::ROLLER_SIGNALS_ONE_ROTATION as f32 / kilos;
        println!("set_duenger: {} {} {}", kilos, guard.name, guard.signals_per_kilo_duenger);
    }

    pub fn start_calculate(&self) -> JoinHandle<()> {

        let buf_wheel = self.signals_wheel.clone();
        let buf_roller = self.signals_roller.clone();
        let settings = self.settings.clone();
        let time_window = self.time_window;

        thread::spawn(move || {

            let mut last_calc : Option<Instant>= None;

            loop {
                thread::sleep(Duration::from_secs(1));

                let now = Instant::now();
                //
                // kilos per hekar
                //
                let kilos_per_ha = calculate_current_kilo_per_ha(
                    buf_wheel.lock().unwrap().signals_within_duration(&now, &time_window),
                    buf_roller.lock().unwrap().signals_within_duration(&now, &time_window),
                    &settings.lock().unwrap().signals_per_kilo_duenger);
                //
                // sum meters, kilos
                //
                let diff = match last_calc {
                    None => Duration::MAX,
                    Some(last) => now - last
                };

                let (meters, kilos) = sum_meters_and_kilos(
                    buf_wheel.lock().unwrap().signals_within_duration(&now, &diff),
                    buf_roller.lock().unwrap().signals_within_duration(&now, &diff),
                    &settings.lock().unwrap().signals_per_kilo_duenger);

                last_calc = Some(now);
            }
        })
    }

    pub fn start_receive_signals(&self, signals_rx: Receiver<SignalKind>) -> JoinHandle<()> {

        let buf_wheel = self.signals_wheel.clone();
        let buf_roller = self.signals_roller.clone();

        thread::spawn( move || {
            for signal in signals_rx.iter() {
                match signal {
                    WHEEL(instant) => {
                        buf_wheel.lock().unwrap().push(instant);
                        //print!("W");
                    },
                    ROLLER(instant) => {
                        buf_roller.lock().unwrap( ).push(instant);
                        //print!(" R ");
                    }
                }
                //std::io::stdout().flush().unwrap();
            }
            println!("SignalProcessor ended");
        })
    }

    pub fn new(time_window : Duration, name_duenger : &str, signals_per_kilo_duenger : &f32) -> SignalProcessor {
        SignalProcessor {
            time_window,
            current: Arc::new(Mutex::new(CurrentValues {
                kg_ha: 0.0,
                sum_meters: 0.0,
                sum_kg: 0.0
            })),
            settings : Arc::new(Mutex::new( DuengerSettings {
                name : name_duenger.to_string(),
                signals_per_kilo_duenger: *signals_per_kilo_duenger
            })),
            signals_wheel: Arc::new(Mutex::new(RingBuffer::new(2048))),
            signals_roller: Arc::new(Mutex::new(RingBuffer::new(2048)))
        }
    }
}
