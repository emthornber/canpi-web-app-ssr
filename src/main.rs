use actix_files as fs;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::collections::HashMap;
use std::process;
use std::sync::Mutex;
use tera::{from_value, to_value, Function, Tera, Value};

mod errors;
mod handlers;
mod models;
mod routes;
mod state;
mod validation;

use canpi_config::*;
use routes::*;
use state::AppState;
use validation::*;

fn make_scope_for<'a>(scopes: &'static HashMap<&'a str, String>) -> impl Function + 'a {
    Box::new(
        move |args: &HashMap<String, Value>| -> tera::Result<Value> {
            match args.get("scope") {
                Some(val) => match from_value::<String>(val.clone()) {
                    Ok(v) => Ok(to_value(scopes.get(&*v).unwrap()).unwrap()),
                    Err(_) => Err("oops err".into()),
                },
                None => Err("oops none".into()),
            }
        },
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    if let Ok(canpi_cfg) = CanpiConfig::new() {
        // Configuration file names
        let autohs_file = canpi_cfg.autohs_ini_path.unwrap();
        let canpi_file = canpi_cfg.canpi_ini_path.unwrap();
        // WEbpage formatting files
        let static_path = canpi_cfg.static_path.unwrap();
        // Create and load the autohs configuration using the JSON schema file
        let mut cfg_autohs = Cfg::new();
        cfg_autohs
            .load_configuration(autohs_file.clone(), canpi_cfg.config_path.clone().unwrap())
            .expect("Cannot load autohs configuration");

        // Create and load the canpi configuration using the JSON schema file
        let mut cfg_canpi = Cfg::new();
        cfg_canpi
            .load_configuration(canpi_file.clone(), canpi_cfg.config_path.clone().unwrap())
            .expect("Cannot load canpi configuration");

        // Start HTTP Server
        let host_port = canpi_cfg.host_port.unwrap();
        let shared_date = web::Data::new(Mutex::new(AppState {
            layout_name: hostname::get()?.into_string().unwrap(),
            project_id: "{project_id}".to_string(),
            autohs_ini_file: autohs_file.clone(),
            canpi_ini_file: canpi_file.clone(),
            autohs_cfg: cfg_autohs,
            canpi_cfg: cfg_canpi,
        }));
        let mut tera = Tera::new(canpi_cfg.template_path.unwrap().as_str()).unwrap();
        tera.register_function("scope_for", make_scope_for(&ROUTE_DATA));
        let app = move || {
            App::new()
                .data(tera.clone())
                .app_data(shared_date.clone())
                .configure(autohs_routes)
                .configure(canpi_routes)
                .configure(general_routes)
                .service(fs::Files::new("/static", static_path.clone()).show_files_listing())
        };
        println!(
            "Listening on: {}, open browser and visit - have a go!",
            host_port
        );
        HttpServer::new(app).bind(&host_port)?.run().await
    } else {
        println!("EV contents failed validation - exiting ...");
        process::exit(1);
    }
}
