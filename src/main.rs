use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

//use std::sync::mpsc::{channel, Receiver, Sender};

mod web_routes;
mod web_handler;

mod signal_processor;
mod ring_buffer;
mod fake_signals;
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
mod gpio_signals;

use signal_processor::{SignalKind, SignalProcessor};

#[tokio::main]
async fn main() {

    let ( signals_tx, signals_rx) = std::sync::mpsc::channel::<SignalKind>();

    let thread_fakesignals = fake_signals::start(signals_tx);

    let processor = SignalProcessor::new(Duration::from_secs(20), "Kali", &(30f32 / 4.1) );
    processor.start_receive_signals(signals_rx);
    processor.start_calculate();


    let port = 8080;
    let ipv4and6 = IpAddr::from_str("::0").unwrap(); //should serve IPv4 AND IPv6
    println!("serving Duenger at :{}", port);
    web_routes::run((ipv4and6, port) , &processor).await;
}