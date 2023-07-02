use actix_web::{web, Error, HttpResponse, Result};
use canpi_config::ActionBehaviour::Edit;
use canpi_config::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Mutex;

use crate::errors::CanPiAppError;
use crate::models::{AttrNameText, EditAttrForm};
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

pub async fn status_autohs(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let app_state = app_state.lock().unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    let s = tmpl
        .render("autohs_index.html", &ctx)
        .map_err(|_| CanPiAppError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn display_autohs(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let mut attributes: Vec<AttrLine> = Vec::new();
    let mut ordered_attr: BTreeMap<String, Attribute> = BTreeMap::new();
    let app_state = app_state.lock().unwrap();
    for (n, v) in app_state
        .autohs_cfg
        .attributes_with_action(ActionBehaviour::Display)
        .iter()
    {
        ordered_attr.insert(n.clone(), v.clone());
    }
    for (n, v) in app_state
        .autohs_cfg
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
        .render("autohs_display.html", &ctx)
        .map_err(|_| CanPiAppError::TeraError("autohs_display.html".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn edit_autohs(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
    attr_id: web::Query<AttrNameText>,
) -> Result<HttpResponse, Error> {
    let mut attributes: Vec<AttrLine> = Vec::new();
    let app_state = app_state.lock().unwrap();
    let attribute = app_state.autohs_cfg.read_attribute(attr_id.name.clone());
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
            .render("autohs_edit.html", &ctx)
            .map_err(|_| CanPiAppError::TeraError("autohs_edit.html".to_string()))?;
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    } else {
        let s = format!("Internal error: {} not found", attr_id.name).to_string();
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    }
}

pub async fn update_autohs(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
    params: web::Form<EditAttrForm>,
) -> Result<HttpResponse, Error> {
    let attr_name = params.name.clone();
    let attr_prompt = params.prompt.clone();
    let current_value = params.value.clone();
    let mut s = "(update_autohs called)".to_string();

    let mut app_state = app_state.lock().unwrap();
    let attr = app_state.autohs_cfg.read_attribute(attr_name.clone());
    if let Some(aref) = attr {
        let mut a = aref.clone();
        a.current = current_value.to_string();
        let _ = app_state.autohs_cfg.write_attribute(attr_name.clone(), &a);
        let mut ctx = tera::Context::new();
        ctx.insert("layout_name", &app_state.layout_name);
        ctx.insert("attr_prompt", &attr_prompt);
        ctx.insert("current_value", &current_value);
        s = tmpl
            .render("autohs_confirm.html", &ctx)
            .map_err(|_| CanPiAppError::TeraError("autohs_confirm.html".to_string()))?;
    } else {
        s = format!("Key {} not in autohs configuration", attr_name);
    }
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn save_autohs(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let mut status_text = "(save_autohs() called)".to_string();
    let app_state = app_state.lock().unwrap();
    let autohs_ini_file = app_state.autohs_ini_file.clone();
    if let Ok(()) = app_state
        .autohs_cfg
        .write_cfg_file(&autohs_ini_file, Some(true))
    {
        status_text = format!("Configuration file {} updated", &autohs_ini_file).to_string();
    } else {
        status_text = format!("Failed to updated {}", &autohs_ini_file).to_string();
    }
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    ctx.insert("status", &status_text);
    let s = tmpl
        .render("autohs_save.html", &ctx)
        .map_err(|_| CanPiAppError::TeraError("autohs_save.html".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[cfg(test)]
mod tests {}
