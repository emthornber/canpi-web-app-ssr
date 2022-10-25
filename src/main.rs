use actix_files as fs;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::sync::Mutex;

mod handlers;
mod errors;
mod models;
mod routes;
mod state;

use tera::Tera;

use canpi_config::*;
use routes::*;
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Define JSON schema file
    let def_file = concat!(env!("CARGO_MANIFEST_DIR"), "/static/canpi-config-defn.json").to_string();

    // Create and load the autohs configuration
    let autohs_ini_file = env::var("AUTOHS_INI_FILE")
        .expect("AUTOHS_INI_FILE path is not set in .env file").to_string();
    let mut cfg_autohs = Cfg::new();
    cfg_autohs.load_configuration(
        autohs_ini_file, def_file.clone())
        .expect("Cannot load autohs configuration");

    // Create and load the canpi configuration
    let canpi_ini_file = env::var("CANPI_INI_FILE")
        .expect("CANPI_INI_FILE path is not set in .env file").to_string();
    let mut cfg_canpi = Cfg::new();
    cfg_canpi.load_configuration(canpi_ini_file, def_file.clone())
        .expect("Cannot load canpi configuration");

    // Start HTTP Server
    let host_port = env::var("HOST_PORT").expect("HOST_PORT address is not set in .env file");
    let shared_date = web::Data::new(Mutex::new(AppState {
        layout_name: hostname::get()?.into_string().unwrap(),
        project_id: "{project_id}".to_string(),
        autohs_cfg: cfg_autohs,
        canpi_cfg: cfg_canpi,
    }));
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    let app = move ||
        App::new()
            .data(tera.clone())
            .app_data(shared_date.clone())
            .configure(autohs_routes)
            .configure(canpi_routes)
            .configure(general_routes)
            .service(fs::Files::new("/static", "./canpi-web-app-ssr/static")
                .show_files_listing());


    println!("Listening on: {}, open browser and visit have a try!", host_port);
    HttpServer::new(app)
        .bind(&host_port)?
        .run()
        .await
}
