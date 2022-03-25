use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::net::{SocketAddr};
use warp::{Filter, Reply, Rejection};
use warp::http::{Response, StatusCode};
use warp::reply::Json;
use bytes::{Bytes};

use crate::{SignalProcessor};
use crate::signal_processor::Duenger;

pub async fn apply_changes(duenger : HashMap<String, String>, proc : SignalProcessor ) -> Result<impl Reply, std::convert::Infallible> {

    let name = duenger.get("name");
    let kg = duenger.get("kg");

    if name.is_none() {
        Ok(warp::reply::with_status("field missing: name", StatusCode::BAD_REQUEST))
    }
    else if kg.is_none() {
        Ok(warp::reply::with_status("field missing: kg", StatusCode::BAD_REQUEST))
    }
    else {
        let duengerKg = match kg.unwrap().parse::<f32>() {
            Err(e) => return Ok(warp::reply::with_status("cannot parse kg to float", StatusCode::BAD_REQUEST)),
            Ok(kg_in_f32) => kg_in_f32
        };
        proc.set_duenger(name.unwrap().as_str(), duengerKg);
        Ok(warp::reply::with_status("ok", StatusCode::OK))
    }
}

pub async fn load_settings(proc : SignalProcessor) -> Result<warp::reply::Response, std::convert::Infallible> {
    match fs::read_to_string(crate::signal_processor::FILENAME_DUENGER_JSON ) {
        Err(e) =>
            Ok(warp::http::Response::builder()
                .status(500)
                .body("duenger.json not found")
                .into_response() ),
        Ok(json_config) => {
            Ok(warp::http::Response::builder()
                .status(200)
                // Are you sure about this one? More like "text/plain"?
                .header("Content-Type", "application/json; charset=utf-8")
                .body(json_config)
                .into_response())
        }
    }
}

pub async fn save_settings(duenger : bytes::Bytes, proc : SignalProcessor ) -> Result<impl Reply, std::convert::Infallible> {
    println!("save_settings");
    match fs::write(crate::signal_processor::FILENAME_DUENGER_JSON, duenger) {
        Err(e) => Ok(warp::reply::with_status(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)),
        Ok(()) => Ok(warp::reply::with_status("ok".to_string(), StatusCode::OK))
    }
}