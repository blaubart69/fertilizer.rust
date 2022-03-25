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

    let get_settings =
        warp::get()
            .and(warp::path("settings") )
            .and( warp::path::end() )
            .and(processor_filter.clone() )
            .and_then( web_handler::load_settings );

    let post_settings =
        warp::post()
            .and(warp::path("settings") )
            .and( warp::path::end() )
            .and( warp::body::bytes() )
            .and(processor_filter.clone() )
            .and_then( web_handler::save_settings );

    let apply_settings =
        warp::post()
            .and( warp::path("applyChanges") )
            .and( warp::path::end() )
            .and(warp::body::json() )
            .and( processor_filter.clone() )
            .and_then( web_handler::apply_changes );

    let static_content =
        warp::get().and(warp::fs::dir("./static"));

    let routes =
                get_settings
            .or(post_settings)
            .or(apply_settings)
            .or(static_content);

        warp::serve(routes).run(listen_socket_addr ).await;
}