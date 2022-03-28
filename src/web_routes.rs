use std::net::{SocketAddr};
use std::sync::{Arc, Mutex};
use warp::{Filter};

use crate::{CurrentSettings, CurrentValues, web_handler};

pub async fn run(listen_socket_addr: impl Into<SocketAddr>, current_settings: Arc<Mutex<CurrentSettings>>, current_values: CurrentValues) {

    let settings_filter = {
        let settings_clone = current_settings.clone();
        warp::any().map(move || settings_clone.clone() )
    };

    let values_filter = {
        let values_clone = current_values.clone();
        warp::any().map(move || values_clone.clone() )
    };

    let get_settings =
        warp::get()
            .and(warp::path("settings") )
            .and( warp::path::end() )
            .and_then( web_handler::load_settings );

    let post_settings =
        warp::post()
            .and(warp::path("settings") )
            .and( warp::path::end() )
            .and( warp::body::bytes() )
            .and_then( web_handler::save_settings );

    let apply_settings =
        warp::post()
            .and( warp::path("applyChanges") )
            .and( warp::path::end() )
            .and(warp::body::json() )
            .and( settings_filter.clone() )
            .and_then( web_handler::apply_changes );

    let get_current =
        warp::get()
            .and( warp::path("current") )
            .and( warp::path::end() )
            .and( settings_filter.clone() )
            .and ( values_filter.clone() )
            .and_then( web_handler::current );

    let static_content =
        warp::get().and(warp::fs::dir("./static"));

    let routes =
                get_settings
            .or(post_settings)
            .or(apply_settings)
            .or(get_current)
            .or(static_content);

        warp::serve(routes).run(listen_socket_addr ).await;
}