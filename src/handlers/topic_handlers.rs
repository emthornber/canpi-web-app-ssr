use actix_web::{web, Error, HttpResponse, Result};
use canpi_config::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::errors::CanPiAppError;
use crate::models::{AttrNameText, EditAttrForm};
use crate::state::AppState;
use crate::topics::build_topic_menu_html;
// use crate::validation::CanpiConfig;

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

pub fn get_attr_defns(app_state: &AppState) -> Result<&Cfg, Error> {
    if let Some(topic) = &app_state.current_topic {
        let attr_defn = &topic.attr_defn;
        return Ok(&attr_defn);
    }
    Err(
        CanPiAppError::NotFound("Cannot read attribute definitions for current_topic".to_string())
            .into(),
    )
}

pub fn get_mut_attr_defns(app_state: &mut AppState) -> Result<&mut Cfg, Error> {
    if let Some(topic) = &mut app_state.current_topic {
        let attr_defn: &mut canpi_config::Cfg = &mut topic.attr_defn;
        return Ok(attr_defn);
    }
    Err(
        CanPiAppError::NotFound("Cannot read attribute definitions for current_topic".to_string())
            .into(),
    )
}

pub fn get_ini_file_path(app_state: &AppState) -> Result<String, Error> {
    if let Some(topic) = &app_state.current_topic {
        return Ok(topic.ini_file_path.clone());
    }
    Err(
        CanPiAppError::NotFound("Cannot read attribute definitions for current_topic".to_string())
            .into(),
    )
}

pub fn restart_service(app_state: &AppState) -> Result<String, Error> {
    if let Some(topic) = &app_state.current_topic {
        if let Some(service_name) = &topic.service_name {
            if let Ok(()) = topic.restart_topic() {
                return Ok(service_name.clone());
            } else {
                return Err(
                    CanPiAppError::Other("Failed to restart service ".to_string().into()).into(),
                );
            }
        }
        return Err(CanPiAppError::NotFound(
            "Service name not found for current topic".to_string(),
        )
        .into());
    }
    Err(CanPiAppError::NotFound("No topic selected".to_string()).into())
}

pub async fn status_topic(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let app_state = app_state.lock().unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    if let Some(topic) = &app_state.current_topic {
        // Create the topic menu HTML include file
        let tmpl_root = app_state.template_root.clone();
        let mut format_file = PathBuf::from(tmpl_root);
        format_file.push("topic_menu.format");
        if let Ok(()) = build_topic_menu_html(&topic, format_file.as_path()) {
            log::info!("Top menu created")
        } else {
            log::warn!("Failed to create top menu");
        }
        ctx.insert("topic_title", &topic.title);
    } else {
        ctx.insert("topic_title", "No topic selected");
    };
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
        let attr = AttrLine {
            name: n.clone(),
            prompt: v.prompt.clone(),
            tooltip: v.tooltip.clone(),
            value: v.current.clone(),
            default: "".to_string(),
            format: "".to_string(),
            editable: v.action == ActionBehaviour::Edit,
        };
        attributes.push(attr);
    }
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    if let Some(topic) = &app_state.current_topic {
        ctx.insert("topic_title", &topic.title);
    } else {
        ctx.insert("topic_title", "No topic selected");
    };
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
        if let Some(topic) = &app_state.current_topic {
            ctx.insert("topic_title", &topic.title);
        } else {
            ctx.insert("topic_title", "No topic selected");
        };
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

    let mut app_state = app_state.lock().unwrap();
    let attr_defn = get_mut_attr_defns(&mut app_state)?;
    let attr = attr_defn.read_attribute(attr_name.clone());
    if let Some(aref) = attr {
        let mut a = aref.clone();
        a.current = current_value.to_string();
        let _ = attr_defn.write_attribute(attr_name.clone(), &a);
        let mut ctx = tera::Context::new();
        ctx.insert("layout_name", &app_state.layout_name);
        if let Some(topic) = &app_state.current_topic {
            ctx.insert("topic_title", &topic.title);
        } else {
            ctx.insert("topic_title", "No topic selected");
        };
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
    let topic_ini_file = get_ini_file_path(&app_state)?;
    let attr_defn = get_attr_defns(&app_state)?;
    if let Ok(()) = attr_defn.write_cfg_file(&topic_ini_file, Some(true)) {
        _status_text = format!("Configuration file {} updated", &topic_ini_file).to_string();
    } else {
        _status_text = format!("Failed to update {}", &topic_ini_file).to_string();
    }
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    if let Some(topic) = &app_state.current_topic {
        ctx.insert("topic_title", &topic.title);
    } else {
        ctx.insert("topic_title", "No topic selected");
    };
    ctx.insert("status", &_status_text);
    let s = tmpl
        .render("topic_save.html", &ctx)
        .map_err(|_| CanPiAppError::TeraError("topic_save.html".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn restart_topic(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let mut _status_text = "(restart_topic() called)".to_string();
    let app_state = app_state.lock().unwrap();
    if let Ok(topic_service_name) = restart_service(&app_state) {
        _status_text = format!("Service {} restarted", &topic_service_name).to_string();
    } else {
        _status_text = format!("Failed to restart service").to_string();
    }
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    if let Some(topic) = &app_state.current_topic {
        ctx.insert("topic_title", &topic.title);
    } else {
        ctx.insert("topic_title", "No topic selected");
    };
    ctx.insert("status", &_status_text);
    let s = tmpl
        .render("topic_restart.html", &ctx)
        .map_err(|_| CanPiAppError::TeraError("topic_restart.html".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[cfg(test)]
mod tests {}
