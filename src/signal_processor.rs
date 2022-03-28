use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

use crate::ring_buffer::RingBuffer;

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

#[derive(Clone)]
pub struct CurrentValues {
    pub kg_ha              : Arc<Mutex<f32>>,
    pub sum_signals_wheel  : Arc<AtomicUsize>,
    pub sum_signals_roller : Arc<AtomicUsize>,
}

pub struct CurrentResult {
    pub kg_ha : f32,
    pub sum_kilos : f32,
    pub sum_meters : f32
}

impl CurrentValues {
    pub fn new() -> CurrentValues {
        CurrentValues {
            kg_ha : Arc::new(Mutex::new(0.0)),
            sum_signals_roller: Arc::new(AtomicUsize::new(0)),
            sum_signals_wheel: Arc::new(AtomicUsize::new(0))
        }
    }
    pub fn get_current(&self, signals_per_kilo_duenger : &f32) -> CurrentResult {
        CurrentResult {
            kg_ha      : *self.kg_ha.lock().unwrap(),
            sum_kilos  :  self.sum_signals_roller.load(Ordering::Relaxed) as f32 / signals_per_kilo_duenger,
            sum_meters :  self.sum_signals_wheel.load(Ordering::Relaxed) as f32 / SIGNALS_PER_METER
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Duenger {
    pub name: String,
    pub kg: f32
}

pub struct CurrentSettings {
    pub name                     : String,
    pub signals_per_kilo_duenger : f32
}

impl CurrentSettings {
    pub fn new(name : &str, kilos : f32) -> CurrentSettings {
        let mut init = CurrentSettings {
            name : "".to_string(),
            signals_per_kilo_duenger : 0.0
        };

        init.set_duenger(name,kilos);
        init
    }

    pub fn  set_duenger(&mut self, name : &str, kilos : f32) {
        self.name = name.to_string();
        self.signals_per_kilo_duenger = self::ROLLER_SIGNALS_ONE_ROTATION as f32 / kilos;
        println!("set_duenger: name: {} | kilos: {} | signals_per_kilo: {}", self.name, kilos, self.signals_per_kilo_duenger);
    }
}

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
/*
fn sum_meters_and_kilos(signals_wheel_last_calc : usize, signals_roller_last_calc : usize, signals_per_kilo_duenger : &f32) -> (f32,f32) {

    let meter_since_last_calc  = signals_wheel_last_calc as f32  / SIGNALS_PER_METER;
    let kilos_since_last_calc  = signals_roller_last_calc as f32 / signals_per_kilo_duenger;

    ( meter_since_last_calc, kilos_since_last_calc )
}*/

pub fn start_update_kg_per_ha(
    signals_wheel    : Arc<Mutex<RingBuffer>>,
    signals_roller   : Arc<Mutex<RingBuffer>>,
    settings         : Arc<Mutex<CurrentSettings>>,
    kg_per_ha        : Arc<Mutex<f32>>,
    time_window      : Duration) -> JoinHandle<()> {

    thread::spawn(move || {

        loop {
            thread::sleep(Duration::from_secs(1));

            let now = Instant::now();
            //
            // kilos per hekar within the given timespan
            //
            match calculate_current_kilo_per_ha(
                signals_wheel.lock().unwrap().signals_within_duration(&now, &time_window),
                signals_roller.lock().unwrap().signals_within_duration(&now, &time_window),
                &settings.lock().unwrap().signals_per_kilo_duenger ) {
                    Some(kg_current) => *(kg_per_ha.lock().unwrap()) = kg_current,
                    _ => {}
            }
        }
    })
}

