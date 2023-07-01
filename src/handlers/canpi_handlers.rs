use actix_web::{web, Error, HttpResponse, Result};
use canpi_config::ActionBehaviour::Edit;
use canpi_config::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Mutex;

use crate::errors::CanPiAppError;
use crate::models::EditAttrForm;
use crate::state::AppState;

#[derive(Serialize, Deserialize)]
pub struct AttrLine {
    name: String,
    prompt: String,
    tooltip: String,
    value: String,
    default: String,
    format: String,
    editable: bool,
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
    for (n, v) in app_state
        .canpi_cfg
        .attributes_with_action(ActionBehaviour::Display)
        .iter()
    {
        ordered_attr.insert(n.clone(), v.clone());
    }
    for (n, v) in app_state
        .canpi_cfg
        .attributes_with_action(ActionBehaviour::Edit)
        .iter()
    {
        ordered_attr.insert(n.clone(), v.clone());
    }
    for (n, v) in ordered_attr.iter() {
        let editable = if v.action == Edit { true } else { false };
        let attr = AttrLine {
            name: n.clone(),
            prompt: v.prompt.clone(),
            tooltip: v.tooltip.clone(),
            value: v.current.clone(),
            default: "".to_string(),
            format: "".to_string(),
            editable: editable,
        };
        attributes.push(attr);
    }
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    ctx.insert("configuration", &attributes);
    let s = tmpl
        .render("canpi_display.html", &ctx)
        .map_err(|_| CanPiAppError::TeraError("canpi_display.html".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[derive(Debug, Deserialize)]
pub struct CanpiAttrName {
    name: String,
}

pub async fn edit_canpi(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
    attr_id: web::Query<CanpiAttrName>,
) -> Result<HttpResponse, Error> {
    let mut attributes: Vec<AttrLine> = Vec::new();
    let app_state = app_state.lock().unwrap();
    let attribute = app_state.canpi_cfg.read_attribute(attr_id.name.clone());
    if let Some(v) = attribute {
        let attr = AttrLine {
            name: attr_id.name.clone(),
            prompt: v.prompt.clone(),
            tooltip: v.tooltip.clone(),
            value: v.current.clone(),
            default: v.default.clone(),
            format: v.format.clone(),
            editable: false,
        };
        attributes.push(attr);
        let mut ctx = tera::Context::new();
        ctx.insert("layout_name", &app_state.layout_name);
        ctx.insert("configuration", &attributes);
        let s = tmpl
            .render("canpi_edit.html", &ctx)
            .map_err(|_| CanPiAppError::TeraError("canpi_edit.html".to_string()))?;
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    } else {
        let s = format!("Internal error: {} not found", attr_id.name).to_string();
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    }
}

pub async fn update_canpi(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
    params: web::Form<EditAttrForm>,
) -> Result<HttpResponse, Error> {
    let cts = tera::Context::new();
    let attr_name = params.name.clone();
    let attr_prompt = params.prompt.clone();
    let current_value = params.value.clone();

    let mut app_state = app_state.lock().unwrap();
    let attr = app_state.canpi_cfg.read_attribute(attr_name.clone());
    if let Some(aref) = attr {
        let mut a = aref.clone();
        a.current = current_value.to_string();
        let _ = app_state.canpi_cfg.write_attribute(attr_name.clone(), &a);
        let mut ctx = tera::Context::new();
        ctx.insert("layout_name", &app_state.layout_name);
        ctx.insert("attr_prompt", &attr_prompt);
        ctx.insert("current_value", &current_value);
        let s = tmpl
            .render("canpi_confirm.html", &ctx)
            .map_err(|_| CanPiAppError::TeraError("canpi_confirm.html".to_string()))?;
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    } else {
        let s = format!("Key {} not in canpi configuration", attr_name);
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    }
}

pub async fn confirm_canpi(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let mut s = "(confirm_canpi() called)".to_string();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn save_canpi(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let mut status_text = "(save_canpi() called)".to_string();
    let app_state = app_state.lock().unwrap();
    let canpi_ini_file = app_state.canpi_ini_file.clone();
    if let Ok(()) = app_state
        .canpi_cfg
        .write_cfg_file(&canpi_ini_file, Some(true))
    {
        status_text = format!("Configuration file {} updated", &canpi_ini_file).to_string();
    } else {
        status_text = format!("Failed to updated {}", &canpi_ini_file).to_string();
    }
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    ctx.insert("status", &status_text);
    let s = tmpl
        .render("canpi_save.html", &ctx)
        .map_err(|_| CanPiAppError::TeraError("canpi_save.html".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[cfg(test)]
mod tests {}
