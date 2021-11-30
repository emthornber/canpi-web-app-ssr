use actix_files as fs;
use actix_web::{error, web, App, Error, HttpResponse, HttpServer, Result};
use std::io;
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

use routes::*;
use state::AppState;
use crate::models::WiFiParameters;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_date = web::Data::new(AppState {
        health_check_response: "CANPiServer running.  Checked".to_string(),
        visit_count: Mutex::new(0),
        wifi_params: Mutex::new( WiFiParameters {
            ap_ssid: String::from("holywelltown"),
            ap_passwd: String::from("***********"),
            ap_channel: String::from("6"),
            router_ssid: String::from("BTWholeHome-VFC"),
            router_passwd: String::from("**************"),
        }),
    });
    let app = move || {
        App::new()
            .app_data(shared_date.clone())
            .configure(general_routes)
            .configure(config_routes)
            .service(fs::Files::new("/static", "./canpi-web-app-ssr/static")
                .show_files_listing())
    };

    println!("Listening on: 0.0.0.0:8080, open browser and visit have a try!");
    HttpServer::new(app)
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
