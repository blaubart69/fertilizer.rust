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


//#[tokio::main]
async fn webserver_main(settings : Arc<Mutex<CurrentSettings>>, values : CurrentValues) {
    let port = 8080;
    let ipv4and6 = IpAddr::from_str("::0").unwrap(); //should serve IPv4 AND IPv6
    println!("serving Duenger at :{}", port);
    web_routes::run((ipv4and6, port) , settings, values ).await;
}

fn main() {
    let settings = Arc::new(Mutex::new(CurrentSettings::new("Kali", 4.1)));
    let values = CurrentValues::new();
    let signals_wheel= Arc::new(Mutex::new(RingBuffer::new(2048)));
    let signals_roller= Arc::new(Mutex::new(RingBuffer::new(2048)));

    #[cfg(debug_assertions)]
    fake_signals::start(signals_wheel.clone(), signals_roller.clone());

    #[cfg(not(debug_assertions))]
    let mut gpio: GpioSignals = match GpioSignals::new() {
        Err(e) => {
            eprintln!("GPIO: setup failed. {}", e);
            return;
        },
        Ok(g) => g
    };
    #[cfg(not(debug_assertions))]
    if let Err(e) = gpio.start(signals_wheel.clone(), signals_roller.clone(), values.clone() ) {
        eprintln!("GPIO: registering interrupts failed. {}", e.to_string());
        return;
    }

    signal_processor::start_update_kg_per_ha(
        signals_wheel.clone(),
        signals_roller.clone(),
        settings.clone(),
        values.kg_ha.clone(),
        Duration::from_secs(20));

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(webserver_main(settings, values.clone() ) );
}