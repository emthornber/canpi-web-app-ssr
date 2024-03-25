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

pub fn get_attr_defns(app_state: &AppState) -> Result<Cfg, Error> {
    if let Some(title) = &app_state.current_topic {
        if let Some(topic) = app_state.topics.get(title) {
            return Ok(topic.attr_defn);
        }
    }
    Err(CanPiAppError::NotFound(
        "Cannot read attribute definitions for {topics.current_topic}".to_string(),
    )
    .into())
}
pub fn get_ini_file_path(app_state: &AppState) -> Result<String, Error> {
    if let Some(title) = &app_state.current_topic {
        if let Some(topic) = app_state.topics.get(title) {
            return Ok(topic.ini_file_path.clone());
        }
    }
    Err(CanPiAppError::NotFound(
        "Cannot read attribute definitions for {topics.current_topic}".to_string(),
    )
    .into())
}

pub async fn status_topic(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let app_state = app_state.lock().unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    ctx.insert("topic_title", &app_state.current_topic);
    let s = tmpl
        .render("topic_index.html", &ctx)
        .map_err(|_| CanPiAppError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn display_topic(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let mut attributes: Vec<AttrLine> = Vec::new();
    let mut ordered_attr: BTreeMap<String, Attribute> = BTreeMap::new();
    let app_state = app_state.lock().unwrap();
    let attr_defn = get_attr_defns(&app_state)?;
    for (n, v) in attr_defn
        .attributes_with_action(ActionBehaviour::Display)
        .iter()
    {
        ordered_attr.insert(n.clone(), v.clone());
    }
    for (n, v) in attr_defn
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
    ctx.insert("topic_title", &app_state.current_topic);
    ctx.insert("configuration", &attributes);
    let s = tmpl
        .render("topic_display.html", &ctx)
        .map_err(|_| CanPiAppError::TeraError("topic_display.html".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn edit_topic(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
    attr_id: web::Query<AttrNameText>,
) -> Result<HttpResponse, Error> {
    let mut attributes: Vec<AttrLine> = Vec::new();
    let app_state = app_state.lock().unwrap();
    let attr_defn = get_attr_defns(&app_state)?;
    let attribute = attr_defn.read_attribute(attr_id.name.clone());
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
        ctx.insert("topic_title", &app_state.current_topic);
        ctx.insert("configuration", &attributes);
        let s = tmpl
            .render("topic_edit.html", &ctx)
            .map_err(|_| CanPiAppError::TeraError("topic_edit.html".to_string()))?;
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    } else {
        let s = format!("Internal error: {} not found", attr_id.name).to_string();
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    }
}

pub async fn update_topic(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
    params: web::Form<EditAttrForm>,
) -> Result<HttpResponse, Error> {
    let attr_name = params.name.clone();
    let attr_prompt = params.prompt.clone();
    let current_value = params.value.clone();
    let mut _s = "(update_topic called)".to_string();

    let app_state = app_state.lock().unwrap();
    let mut attr_defn = get_attr_defns(&app_state)?;
    let attr = attr_defn.read_attribute(attr_name.clone());
    if let Some(aref) = attr {
        let mut a = aref.clone();
        a.current = current_value.to_string();
        let _ = attr_defn.write_attribute(attr_name.clone(), &a);
        let mut ctx = tera::Context::new();
        ctx.insert("layout_name", &app_state.layout_name);
        ctx.insert("topic_title", &app_state.current_topic);
        ctx.insert("attr_prompt", &attr_prompt);
        ctx.insert("current_value", &current_value);
        _s = tmpl
            .render("topic_confirm.html", &ctx)
            .map_err(|_| CanPiAppError::TeraError("topic_confirm.html".to_string()))?;
    } else {
        _s = format!("Key {} not in canpi configuration", attr_name);
    }
    Ok(HttpResponse::Ok().content_type("text/html").body(_s))
}

pub async fn save_topic(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let mut _status_text = "(save_topic() called)".to_string();
    let app_state = app_state.lock().unwrap();
    let attr_defn = get_attr_defns(&app_state)?;
    let topic_ini_file = get_ini_file_path(&app_state)?;
    if let Ok(()) = attr_defn.write_cfg_file(&topic_ini_file, Some(true)) {
        _status_text = format!("Configuration file {} updated", &topic_ini_file).to_string();
    } else {
        _status_text = format!("Failed to updated {}", &topic_ini_file).to_string();
    }
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    ctx.insert("topic_title", &app_state.current_topic);
    ctx.insert("status", &_status_text);
    let s = tmpl
        .render("topic_save.html", &ctx)
        .map_err(|_| CanPiAppError::TeraError("topic_save.html".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[cfg(test)]
mod tests {}
