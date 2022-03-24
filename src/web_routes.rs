use std::collections::HashMap;
use std::net::{SocketAddr};
use warp::{Filter};
use warp::http::StatusCode;
use warp::reply::Json;

use crate::{SignalProcessor};
use crate::signal_processor::Duenger;

pub async fn run(listen_socket_addr: impl Into<SocketAddr>, processor : &SignalProcessor) {

    let processor_filter = {
        let processor_clone = processor.clone();
        warp::any().map(move || processor_clone.clone() )
    };

    let settings_get =
        warp::path("settings")
            .and( processor_filter.clone() )
            .map(move |proc : SignalProcessor| {

                let fixed_settings :Vec<Duenger> = vec![
                    Duenger { name : "Kali".to_string(), kg : 5.1 },
                    Duenger { name : "Lulu".to_string(), kg : 4 as f32 },
                ];

                warp::reply::json(&fixed_settings)
            });

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

    let static_content =
        warp::get().and(warp::fs::dir("./static"));

    let routes =
        settings_get
            .or(static_content)
            .or(apply_settings_post);

    warp::serve(routes).run(listen_socket_addr );
}