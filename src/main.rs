use warp::Filter;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Duenger {
    pub name: String,
    pub kg: f32
}

#[tokio::main]
async fn main() {

    let fixed_settings :Vec<Duenger> = vec![
        Duenger { name : "Kali".to_string(), kg : 5.1 },
        Duenger { name : "Lulu".to_string(), kg : 4 as f32 },
    ];

    let settings_get =
        warp::path("settings")
        .map(move || {
            warp::reply::json(&fixed_settings)
        });

    let port = 8080;
    println!("serving Duenger at :{}", port);
    warp::serve(settings_get)
        .run(([0, 0, 0, 0], port))
        .await;
}