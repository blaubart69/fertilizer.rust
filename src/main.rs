use std::net::IpAddr;
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{channel, Receiver, Sender};

mod signal_processor;
mod ring_buffer;
mod fake_signals;
mod gpio_signals;

use signal_processor::{SignalKind, SignalProcessor};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Duenger {
    pub name: String,
    pub kg: f32
}

#[tokio::main]
async fn main() {



    let processor = SignalProcessor::new(tx, Duration::from_secs(20) );

    let fixed_settings :Vec<Duenger> = vec![
        Duenger { name : "Kali".to_string(), kg : 5.1 },
        Duenger { name : "Lulu".to_string(), kg : 4 as f32 },
    ];

    let settings_get =
        warp::path("settings")
        .map(move || {
            warp::reply::json(&fixed_settings)
        });

    let static_content =
        warp::get().and(warp::fs::dir("./static"));

    let routes =
             settings_get
        .or(static_content);

    let port = 8080;
    println!("serving Duenger at :{}", port);

    //should serve IPv4 AND IPv6
    let ipv4and6 = IpAddr::from_str("::0").unwrap();

    warp::serve(routes).run((ipv4and6, port)).await;
}