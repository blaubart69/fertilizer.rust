use std::collections::HashMap;
use std::fmt::Error;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use warp::Filter;
use serde::{Deserialize, Serialize};
use warp::http::StatusCode;
//use std::sync::mpsc::{channel, Receiver, Sender};

mod signal_processor;
mod ring_buffer;
mod fake_signals;
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
mod gpio_signals;

use signal_processor::{SignalKind, SignalProcessor};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Duenger {
    pub name: String,
    pub kg: f32
}

#[tokio::main]
async fn main() {

    let ( signals_tx, signals_rx) = std::sync::mpsc::channel::<SignalKind>();

    let thread_fakesignals = fake_signals::start(signals_tx);

    let processor = SignalProcessor::new(Duration::from_secs(20), "Kali", &(30f32 / 4.1) );
    processor.start_receive_signals(signals_rx);
    processor.start_calculate();

    let fixed_settings :Vec<Duenger> = vec![
        Duenger { name : "Kali".to_string(), kg : 5.1 },
        Duenger { name : "Lulu".to_string(), kg : 4 as f32 },
    ];

    let settings_get =
        warp::path("settings")
        .map(move || {
            warp::reply::json(&fixed_settings)
        });

    let processor_filter = {
        let processor_clone = processor.clone();
        warp::any().map(move || processor_clone.clone() )
    };
    let apply_settings_post =
        warp::post()
            .and( warp::path("applyChanges") )
            .and( warp::path::end() )
            .and( warp::body::json() )
            .and( processor_filter.clone() )
            .map( |simple_map: HashMap<String, String>, proc : SignalProcessor | {
                match simple_map.get("fertilizerToSet") {
                    None => StatusCode::BAD_REQUEST,
                    Some(duengername) => {
                        proc.set_duenger(duengername.as_str(), 30,5.15);
                        StatusCode::OK
                    }
                }
            });

    processor.set_duenger("bumsti", 30, 4.5);

    let static_content =
        warp::get().and(warp::fs::dir("./static"));

    let routes =
        settings_get
        .or(static_content)
        .or(apply_settings_post);

    let port = 8080;
    println!("serving Duenger at :{}", port);

    //should serve IPv4 AND IPv6
    let ipv4and6 = IpAddr::from_str("::0").unwrap();

    warp::serve(routes).run((ipv4and6, port)).await;
}