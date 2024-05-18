use actix_files as fs;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use std::path::Path;
use std::process;
use std::sync::Mutex;
use tera::{from_value, to_value, Function, Tera, Value};
use time::macros::format_description;

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
        let static_path = canpi_cfg.static_path.unwrap();

        // Create and load the configurations using the JSON schema files
        let topic_hash = load_pkg_cfgs(&canpi_cfg.pkg_defn.unwrap());

        // Create the top menu HTML include file
        if let Some(tmpl_path) = canpi_cfg.template_path.clone() {
            let template_grandparent = Path::new(&tmpl_path)
                .parent()
                .and_then(Path::parent)
                .unwrap();
            let mut format_file = template_grandparent.to_path_buf();
            format_file.push("top_menu.format");
            if let Ok(()) = build_top_menu_html(&topic_hash, format_file.as_path()) {
                log::info!("Top menu created")
            } else {
                log::warn!("Failed to create top menu");
            }
        } else {
            log::warn!("Cannot find top menu format file");
        }
        // Start HTTP Server
        let host_port = canpi_cfg.host_port.unwrap();
        let shared_data = web::Data::new(Mutex::new(AppState {
            layout_name: hostname::get()?.into_string().unwrap(),
            project_id: "{project_id}".to_string(),
            current_topic: None,
            topics: topic_hash,
        }));
        let mut tera = Tera::new(canpi_cfg.template_path.unwrap().as_str()).unwrap();
        tera.register_function("scope_for", make_scope_for(&ROUTE_DATA));
        let app = move || {
            App::new()
                .app_data(web::Data::new(tera.clone()))
                .app_data(shared_data.clone())
                .configure(topic_routes)
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
