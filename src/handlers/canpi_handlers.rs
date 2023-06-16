use actix_web::{web, Error, HttpResponse, Result};
use canpi_config::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Mutex;

use crate::state::AppState;
use crate::errors::CanPiAppError;
use crate::models::CanPiConfigForm;

#[derive(Serialize, Deserialize)]
pub struct AttrLine {
    name: String,
    prompt: String,
    tooltip: String,
    value: String,
    default: String,
    format: String,
}

pub async fn status_canpi(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let app_state = app_state.lock().unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    let s = tmpl
        .render("canpi_index.html", &ctx)
        .map_err(|_| CanPiAppError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn display_canpi(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let mut attributes: Vec<AttrLine> = Vec::new();
    let mut ordered_attr: BTreeMap<String, Attribute> = BTreeMap::new();
    let app_state = app_state.lock().unwrap();
    for ( n, v ) in app_state.canpi_cfg.attributes_with_action(ActionBehaviour::Display).iter() {
        ordered_attr.insert(n.clone(), v.clone());
    }
    for ( n, v ) in app_state.canpi_cfg.attributes_with_action(ActionBehaviour::Edit).iter() {
        ordered_attr.insert(n.clone(), v.clone());
    }
    for ( n, v ) in ordered_attr.iter() {
        let attr = AttrLine {
            name: n.clone(),
            prompt: v.prompt.clone(),
            tooltip: v.tooltip.clone(),
            value: v.current.clone(),
            default: "".to_string(),
            format: "".to_string(),
        };
        attributes.push(attr );
    }
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    ctx.insert("configuration", &attributes);
    let s = tmpl
        .render("canpi_display.html", &ctx)
        .map_err(|_| CanPiAppError::TeraError("canpi_display.html".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn edit_canpi(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let mut attributes: Vec<AttrLine> = Vec::new();
    let mut ordered_attr: BTreeMap<String, Attribute> = BTreeMap::new();
    let app_state = app_state.lock().unwrap();
    for ( n, v ) in app_state.canpi_cfg.attributes_with_action(ActionBehaviour::Edit).iter() {
        ordered_attr.insert(n.clone(), v.clone());
    }
    for ( n, v ) in ordered_attr.iter() {
        let attr = AttrLine {
            name: n.clone(),
            prompt: v.prompt.clone(),
            tooltip: v.tooltip.clone(),
            value: v.current.clone(),
            default: v.default.clone(),
            format: v.format.clone(),
        };
        attributes.push(attr );
    }
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    ctx.insert("configuration", &attributes);
    let s = tmpl
        .render("canpi_edit.html", &ctx)
        .map_err(|_| CanPiAppError::TeraError("canpi_edit.html".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn update_canpi(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
    params: web::Form<CanPiConfigForm>,
) -> Result<HttpResponse, Error> {
    let mut cts = tera::Context::new();
    let s = "(update_canpi called)".to_string();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
#[cfg(test)]
mod tests {
}