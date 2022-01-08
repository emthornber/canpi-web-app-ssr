use actix_web::{web, Error, HttpResponse, Result};
use ini::Ini;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use tera::Tera;

use crate::state::AppState;
use crate::errors::CanPiAppError;

pub async fn status_handler(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let mut app_state = app_state.lock().unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    let s = tmpl
        .render("index.html", &ctx)
        .map_err(|_| CanPiAppError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[derive(Serialize, Deserialize)]
pub struct Attribute {
    name: String,
    value: String,
}

pub async fn display_config(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let mut attributes: HashMap<String, String> = HashMap::new();
    let mut app_state = app_state.lock().unwrap();
    for ( n, v ) in app_state.canpi_cfg.general_section().iter() {
        attributes.insert(String::from(n), String::from(v));
    }
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    ctx.insert("configuration", &attributes);
    let s = tmpl
        .render("list_config.html", &ctx)
        .map_err(|_| CanPiAppError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[cfg(test)]
mod tests {
}