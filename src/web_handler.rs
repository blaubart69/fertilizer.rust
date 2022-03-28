use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use warp::{Reply};
use warp::http::{StatusCode};
use serde::Serialize;
use crate::{CurrentSettings, CurrentValues};

pub async fn apply_changes(duenger : HashMap<String, String>, settings : Arc<Mutex<CurrentSettings>> ) -> Result<impl Reply, std::convert::Infallible> {

    let name = duenger.get("name");
    let kg = duenger.get("kg");

    if name.is_none() {
        Ok(warp::reply::with_status("field missing: name", StatusCode::BAD_REQUEST))
    }
    else if kg.is_none() {
        Ok(warp::reply::with_status("field missing: kg", StatusCode::BAD_REQUEST))
    }
    else {
        let duenger_kg = match kg.unwrap().parse::<f32>() {
            Err(_e) => return Ok(warp::reply::with_status("cannot parse kg to float", StatusCode::BAD_REQUEST)),
            Ok(kg_in_f32) => kg_in_f32
        };
        settings.lock().unwrap().set_duenger(name.unwrap().as_str(), duenger_kg);
        Ok(warp::reply::with_status("ok", StatusCode::OK))
    }
}

pub async fn load_settings() -> Result<warp::reply::Response, std::convert::Infallible> {
    match fs::read_to_string(crate::signal_processor::FILENAME_DUENGER_JSON ) {
        Err(_e) =>
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

pub async fn save_settings(duenger : bytes::Bytes) -> Result<impl Reply, std::convert::Infallible> {
    println!("save_settings");
    match fs::write(crate::signal_processor::FILENAME_DUENGER_JSON, duenger) {
        Err(e) => Ok(warp::reply::with_status(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)),
        Ok(()) => Ok(warp::reply::with_status("ok".to_string(), StatusCode::OK))
    }
}

#[derive(Serialize)]
struct CurrentJson {
    name : String,
    kg_per_ha : f32,
    sum_kg : f32,
    sum_meter : f32
}

pub async fn current(settings : Arc<Mutex<CurrentSettings>>, values : CurrentValues) -> Result<impl Reply, std::convert::Infallible> {
    println!("current");

    let v = values.get_current(&settings.lock().unwrap().signals_per_kilo_duenger);

    let current_reply = CurrentJson {
      name : settings.lock().unwrap().name.to_string(),
        kg_per_ha : v.kg_ha,
        sum_kg : v.sum_kilos,
        sum_meter : v.sum_meters
    };

    Ok(warp::reply::json(&current_reply))
}

