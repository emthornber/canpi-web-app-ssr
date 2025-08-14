use actix_files as fs;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process;
use std::sync::Mutex;
use tera::{from_value, to_value, Function, Tera, Value};
use time::macros::format_description;

use canpi_config::PackageHash;

mod errors;
mod handlers;
mod models;
mod routes;
mod state;
mod topics;
mod validation;

use routes::*;
use state::AppState;
use topics::*;
use validation::*;

use crate::errors::CanPiAppError;

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

fn get_configured_packages(canpi_cfg: &CanpiConfig) -> Result<PackageHash, CanPiAppError> {
    let packages = &canpi_cfg.pkg_defn.packages;
    match packages {
        Some(packages) => {
            log::info!("Loaded {} packages from configuration", packages.len());
            return Ok(packages.clone());
        }
        None => Err(CanPiAppError::NotFound(format!(
            "No packages found in configuration"
        ))),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .env()
        .with_timestamp_format(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second]"
        ))
        .init()
        .unwrap();
    log::info!("canpi webapp started");

    if let Ok(canpi_cfg) = CanpiConfig::new() {
        // Webpage formatting files
        let static_path = canpi_cfg.static_path.clone();

        // Create and load the configurations using the JSON schema files
        if let Ok(package_hash) = get_configured_packages(&canpi_cfg) {
            // Create the top menu HTML include file
            let mut format_file = PathBuf::from(canpi_cfg.template_root.clone());
            format_file.push("top_menu.format");
            if let Ok(()) = build_top_menu_html(&package_hash, format_file.as_path()) {
                log::info!("Top menu created")
            } else {
                log::warn!("Failed to create top menu");
            }
            // Start HTTP Server
            let host_port = canpi_cfg.host_port;
            let shared_data = web::Data::new(Mutex::new(AppState {
                layout_name: hostname::get()?.into_string().unwrap(),
                project_id: "{project_id}".to_string(),
                template_root: canpi_cfg.template_root.clone(),
                current_topic: None,
                packages: package_hash,
            }));
            let mut tera = Tera::new(canpi_cfg.template_path.as_str()).unwrap();
            tera.register_function("scope_for", make_scope_for(&ROUTE_DATA));
            let app = move || {
                App::new()
                    .app_data(web::Data::new(tera.clone()))
                    .app_data(shared_data.clone())
                    .configure(topic_routes)
                    .configure(general_routes)
                    .service(web::scope("/").service(web::redirect("", "layout")))
                    .service(fs::Files::new("/static", static_path.clone()).show_files_listing())
            };
            log::info!("Listening on: {}", host_port);
            HttpServer::new(app).bind(&host_port)?.run().await
        } else {
            log::error!("Failed to load packages configuration");
            process::exit(1);
        }
    } else {
        log::error!("EV contents failed validation - exiting ...");
        process::exit(1);
    }
}
