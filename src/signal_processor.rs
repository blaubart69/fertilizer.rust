use std::io::Write;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use warp::header::value;

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

pub struct CurrentValues {
    pub kg_ha : f32,
    pub sum_meters : f32,
    pub sum_kg : f32,
}

impl CurrentValues {
    pub fn new() -> CurrentValues {
        CurrentValues {
            kg_ha : 0.0,
            sum_kg : 0.0,
            sum_meters : 0.0
        }
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Duenger {
    pub name: String,
    pub kg: f32
}

pub struct CurrentSettings {
    pub name : String,
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
        println!("set_duenger: name: {} kilos: {} signals_per_kilo: {}", self.name, kilos, self.signals_per_kilo_duenger);
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

fn sum_meters_and_kilos(signals_wheel_last_calc : usize, signals_roller_last_calc : usize, signals_per_kilo_duenger : &f32) -> (f32,f32) {

    let meter_since_last_calc  = signals_wheel_last_calc as f32  / SIGNALS_PER_METER;
    let kilos_since_last_calc  = signals_roller_last_calc as f32 / signals_per_kilo_duenger;

    ( meter_since_last_calc, kilos_since_last_calc )
}

pub fn start(signals_wheel : Arc<Mutex<RingBuffer>>,
             signals_roller : Arc<Mutex<RingBuffer>>,
             currentSettings : Arc<Mutex<CurrentSettings>>,
             currentValues : Arc<Mutex<CurrentValues>>,
             time_window : Duration) -> JoinHandle<()> {

    let buf_wheel = signals_wheel.clone();
    let buf_roller = signals_roller.clone();
    let settings = currentSettings.clone();
    let values = currentValues.clone();

    thread::spawn(move || {

        let mut last_calc : Option<Instant>= None;

        loop {
            thread::sleep(Duration::from_secs(1));

            let now = Instant::now();
            //
            // kilos per hekar
            //
            match calculate_current_kilo_per_ha(
                buf_wheel.lock().unwrap().signals_within_duration(&now, &time_window),
                buf_roller.lock().unwrap().signals_within_duration(&now, &time_window),
                &settings.lock().unwrap().signals_per_kilo_duenger) {
                    Some(kg_per_ha) => values.lock().unwrap().kg_ha = kg_per_ha,
                    _ => {}
            }
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

            values.lock().unwrap().sum_meters += meters;
            values.lock().unwrap().sum_kg     += kilos;

            last_calc = Some(now);
        }
    })
}

