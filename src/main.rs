use actix_files as fs;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use errors::CanPiAppError;
use log;
use std::collections::HashMap;
use std::path::Path;
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
use state::{AppState, Topic, TopicHash};
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

fn convert_package_to_topic( pkg: &Package) -> Result<Topic, CanPiAppError> {
    let ini_path = pkg.cfg_path.clone() + "/" + pkg.ini_file.as_str();
    if Path::new(&ini_path).is_file() {
        let json_path = pkg.cfg_path + "/" + pkg.json_file.as_str();
        if Path::new(&json_path).is_file() {
            let mut cfg = Cfg::new();
            cfg.load_configuration(json_path);
            let topic = Topic {
                ini_file_path: ini_path,
                attr_defn: cfg,
            };
            return Ok(topic);
        } else {
            return Err(CanPiAppError::NotFound(
                format!("Json file '{json_path}' not found"),
            ));
        }
    } else {
        return Err(CanPiAppError::NotFound(
            format!("Configuration file '{ini_path}' not found"),
        ));
    }
}

fn load_pkg_cfgs(pkg_defn: &Pkg) -> TopicHash {
    let mut topics = TopicHash::new();
    if let Some(pkg_hash) = pkg_defn.packages {
        for (k, v) in pkg_hash.iter() {
            if let Ok(attr) = convert_package_to_topic(v) {
                topics.insert(k.to_string(), attr);
            }
        } 
    }
    if topics.is_empty() {
        log::warn!("No package attribute definitions found");
    }
    topics
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    if let Ok(canpi_cfg) = CanpiConfig::new() {
        // Webpage formatting files
        let static_path = canpi_cfg.static_path.unwrap();

        // Start HTTP Server
        let host_port = canpi_cfg.host_port.unwrap();
        let shared_data = web::Data::new(Mutex::new(AppState {
            layout_name: hostname::get()?.into_string().unwrap(),
            project_id: "{project_id}".to_string(),
            // Create and load the configurations using the JSON schema files
            topics: load_pkg_cfgs(canpi_cfg.pkg_defn),
        }));
        let mut tera = Tera::new(canpi_cfg.template_path.unwrap().as_str()).unwrap();
        tera.register_function("scope_for", make_scope_for(&ROUTE_DATA));
        let app = move || {
            App::new()
                .data(tera.clone())
                .app_data(shared_data.clone())
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
