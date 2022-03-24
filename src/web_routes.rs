use std::collections::HashMap;
use std::net::{SocketAddr};
use warp::{Filter};
use warp::http::StatusCode;
use warp::reply::Json;

use crate::{SignalProcessor};
use crate::signal_processor::Duenger;
use crate::web_handler;

pub async fn run(listen_socket_addr: impl Into<SocketAddr>, processor : &SignalProcessor) {

    let processor_filter = {
        let processor_clone = processor.clone();
        warp::any().map(move || processor_clone.clone() )
    };

    let settings_get =
        warp::get()
            .and(warp::path("settings") )
            .and( warp::path::end() )
            .and(processor_filter.clone() )
            .and_then( web_handler::get_settings );

    let apply_settings_post =
        warp::post()
            .and( warp::path("applyChanges") )
            .and( warp::path::end() )
            .and( warp::body::json() )
            .and( processor_filter.clone() )
            .and_then( web_handler::apply_changes );
            //.map( |simple_map: HashMap<String, String>, proc : SignalProcessor | {
            //});

    let static_content =
        warp::get().and(warp::fs::dir("./static"));

    let routes =
        settings_get
            .or(static_content)
            .or(apply_settings_post);

    warp::serve(routes).run(listen_socket_addr ).await;
}