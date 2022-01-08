use actix_files as fs;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../errors.rs"]
mod errors;
#[path = "../models.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

use tera::Tera;

use canpi_config::*;
use routes::*;
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Start HTTP Server
    let host_port = env::var("HOST_PORT").expect("HOST_PORT address is not set in .env file");
    let ini_path = env::var("INI_FILE").expect("INI_FILE path is not set in .env file");
    let shared_date = web::Data::new(Mutex::new(AppState {
        layout_name: hostname::get()?.into_string().unwrap(),
        canpi_cfg: read_cfg_file(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), ini_path))
                .expect("Failed to find .ini file"),
    }));
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    let app = move ||
        App::new()
            .data(tera.clone())
            .app_data(shared_date.clone())
            .configure(configure_routes)
            .service(fs::Files::new("/static", "./canpi-web-app-ssr/static")
                .show_files_listing());


    println!("Listening on: {}, open browser and visit have a try!", host_port);
    HttpServer::new(app)
        .bind(&host_port)?
        .run()
        .await
}
