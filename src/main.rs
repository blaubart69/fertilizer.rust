use std::net::IpAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod web_routes;
mod web_handler;

mod signal_processor;
mod ring_buffer;
mod fake_signals;
#[cfg(any(target_arch = "arm", target_arch = "aarch64", target_os = "linux"))]
mod gpio_signals;

use signal_processor::{CurrentSettings};
use crate::gpio_signals::GpioSignals;
use crate::ring_buffer::RingBuffer;
use crate::signal_processor::CurrentValues;


#[tokio::main]
async fn main() {

    let settings = Arc::new(Mutex::new(CurrentSettings::new("Kali", 4.1)));
    let values = Arc::new(Mutex::new(CurrentValues::new()));
    let signals_wheel= Arc::new(Mutex::new(RingBuffer::new(2048)));
    let signals_roller= Arc::new(Mutex::new(RingBuffer::new(2048)));

    #[cfg(debug_assertions)]
    fake_signals::start(signals_wheel.clone(), signals_roller.clone());

    //#[cfg(not(debug_assertions))]
    let mut gpio: GpioSignals = match GpioSignals::new() {
        Err(e) => return (),
        Ok(g) => g
    };
    gpio.start(signals_wheel.clone(), signals_roller.clone());

    signal_processor::start(
        signals_wheel.clone(),
        signals_roller.clone(),
        settings.clone(),
        values.clone(),
        Duration::from_secs(20));

    let port = 8080;
    let ipv4and6 = IpAddr::from_str("::0").unwrap(); //should serve IPv4 AND IPv6
    println!("serving Duenger at :{}", port);
    web_routes::run((ipv4and6, port) , settings.clone(), values.clone() ).await;
}