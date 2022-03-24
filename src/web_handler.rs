use std::collections::HashMap;
use std::fs;
use std::net::{SocketAddr};
use warp::{Filter, Reply, Rejection};
use warp::http::{Response, StatusCode};
use warp::reply::Json;

use crate::{SignalProcessor};
use crate::signal_processor::Duenger;

pub async fn apply_changes(simple_map : HashMap<String, String>, proc : SignalProcessor ) -> Result<impl Reply, std::convert::Infallible> {
    match simple_map.get("fertilizerToSet") {
        None => Ok(StatusCode::BAD_REQUEST),
        Some(duengername) => {
            proc.set_duenger(duengername.as_str(), 30,5.15);
            Ok(StatusCode::OK)
        }
    }
}

pub async fn get_settings(proc : SignalProcessor) -> Result<warp::reply::Response, std::convert::Infallible> {
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