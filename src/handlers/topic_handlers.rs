use actix_web::{web, Error, HttpResponse, Result};
use canpi_config::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Mutex;

use crate::errors::CanPiAppError;
use crate::models::{AttrNameText, EditAttrForm};
use crate::state::AppState;
// use crate::validation::CanpiConfig;

#[derive(Serialize, Deserialize)]
pub struct AttrLine {
    name: String,
    prompt: String,
    tooltip: String,
    value: String,
    default: String,
    // A regular expression to validate user input.
    // If unused this will be an empty string.
    pattern: String,
    // A list of possible values for this attribute.
    // If unused this will be an empty vector.
    select: Vec<String>,
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

// Extract the regexp string from the AttributeFormat.
// If there is no regexp, return an empty string.
pub fn get_pattern_format(format: AttributeFormat) -> String {
    match format {
        AttributeFormat::RegExp(ref re) => return re.clone(),
        _ => return "".to_string(),
    }
}

// Extract the vector from the AttributeFormat.
// If there is no vector, return an empty vector.
pub fn get_select_format(format: AttributeFormat) -> Vec<String> {
    match format {
        AttributeFormat::Vector(ref vec) => return vec.clone(),
        _ => return Vec::new(),
    }
}

/// Check a value sent from the edit form against the attribute's definition.
///
/// The form's pattern is checked only by the browser, and a request can be sent without the
/// form, so the action and format are checked again here before the value is stored.
pub fn check_value(attr: &Attribute, value: &str) -> Result<(), String> {
    if attr.action != ActionBehaviour::Edit {
        return Err("this item cannot be changed".to_string());
    }
    if value.chars().any(|c| c.is_control()) {
        return Err("the value contains a control character".to_string());
    }
    match &attr.format {
        AttributeFormat::RegExp(re) => match Regex::new(&format!("^(?:{})$", re)) {
            Ok(r) if r.is_match(value) => Ok(()),
            Ok(_) => Err(format!("the value does not match the pattern {}", re)),
            Err(_) => Err(format!("the pattern {} in the definition is not valid", re)),
        },
        AttributeFormat::Vector(options) => {
            if options.iter().any(|o| o == value) {
                Ok(())
            } else {
                Err("the value is not one of the choices".to_string())
            }
        }
    }
}

pub fn topic_restart(app_state: &AppState) -> Result<String, Error> {
    if let Some(topic) = &app_state.current_topic {
        if let Ok(service_name) = topic.restart_service() {
            return Ok(service_name.clone());
        } else {
            return Err(
                CanPiAppError::Other("Failed to restart service ".to_string().into()).into(),
            );
        }
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
    ctx.insert("menu_items", &app_state.topic_menu);
    if let Some(topic) = &app_state.current_topic {
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
            pattern: "".to_string(),
            select: Vec::new(),
            editable: v.action == ActionBehaviour::Edit,
        };
        attributes.push(attr);
    }
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    ctx.insert("menu_items", &app_state.topic_menu);
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
            pattern: get_pattern_format(v.format.clone()),
            select: get_select_format(v.format.clone()),
            editable: false,
        };
        attributes.push(attr);
        let mut ctx = tera::Context::new();
        ctx.insert("layout_name", &app_state.layout_name);
        ctx.insert("menu_items", &app_state.topic_menu);
        if let Some(topic) = &app_state.current_topic {
            ctx.insert("topic_title", &topic.title);
        } else {
            ctx.insert("topic_title", "No topic selected");
        };
        ctx.insert("configuration", &attributes);
        // Choose the edit template based on whether the attribute uses regexp or vector.
        let mut edit_html_file = "topic_edit_regexp.html";
        match v.format {
            AttributeFormat::Vector(_) => edit_html_file = "topic_edit_vector.html",
            _ => (),
        }
        let s = tmpl
            .render(edit_html_file, &ctx)
            .map_err(|_| CanPiAppError::TeraError(edit_html_file.to_string()))?;
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
        if let Err(reason) = check_value(aref, &current_value) {
            log::warn!("{} not changed: {}", attr_name, reason);
            _s = format!("{} not changed: {}", aref.prompt, reason);
            return Ok(HttpResponse::BadRequest()
                .content_type("text/html")
                .body(_s));
        }
        let mut a = aref.clone();
        a.current = current_value.to_string();
        let _ = attr_defn.write_attribute(attr_name.clone(), &a);
        let mut ctx = tera::Context::new();
        ctx.insert("layout_name", &app_state.layout_name);
        ctx.insert("menu_items", &app_state.topic_menu);
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
    ctx.insert("menu_items", &app_state.topic_menu);
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
    if let Ok(topic_service_name) = topic_restart(&app_state) {
        _status_text = format!("Service {} restarted", &topic_service_name).to_string();
    } else {
        _status_text = format!("Failed to restart service").to_string();
    }
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    ctx.insert("menu_items", &app_state.topic_menu);
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
mod tests {
    use super::*;

    fn attribute(format: AttributeFormat, action: ActionBehaviour) -> Attribute {
        Attribute {
            prompt: "Port".to_string(),
            tooltip: "".to_string(),
            current: "5555".to_string(),
            default: "5555".to_string(),
            format,
            action,
        }
    }

    #[test]
    fn check_value_pattern() {
        let a = attribute(
            AttributeFormat::RegExp("[0-9]{4}".to_string()),
            ActionBehaviour::Edit,
        );
        assert!(check_value(&a, "5550").is_ok());
        assert!(check_value(&a, "555").is_err());
        assert!(
            check_value(&a, "55500").is_err(),
            "pattern must match the whole value"
        );
        assert!(check_value(&a, "5550;reboot").is_err());
        assert!(check_value(&a, "").is_err());
    }

    #[test]
    fn check_value_alternatives_anchored() {
        let a = attribute(
            AttributeFormat::RegExp("[1-9]|1[0-3]".to_string()),
            ActionBehaviour::Edit,
        );
        assert!(check_value(&a, "6").is_ok());
        assert!(check_value(&a, "13").is_ok());
        assert!(check_value(&a, "14").is_err());
        assert!(check_value(&a, "6x").is_err());
    }

    #[test]
    fn check_value_choices() {
        let a = attribute(
            AttributeFormat::Vector(vec!["WARN".to_string(), "INFO".to_string()]),
            ActionBehaviour::Edit,
        );
        assert!(check_value(&a, "INFO").is_ok());
        assert!(check_value(&a, "DEBUG").is_err());
    }

    #[test]
    fn check_value_only_editable_items() {
        let pattern = AttributeFormat::RegExp(".*".to_string());
        assert!(check_value(&attribute(pattern.clone(), ActionBehaviour::Display), "x").is_err());
        assert!(check_value(&attribute(pattern.clone(), ActionBehaviour::Hide), "x").is_err());
        assert!(check_value(&attribute(pattern, ActionBehaviour::Edit), "x").is_ok());
    }

    #[test]
    fn check_value_rejects_control_characters() {
        let a = attribute(
            AttributeFormat::RegExp(".*".to_string()),
            ActionBehaviour::Edit,
        );
        assert!(check_value(&a, "a\nb=1").is_err());
        assert!(check_value(&a, "a\rb").is_err());
    }

    #[test]
    fn check_value_invalid_pattern() {
        let a = attribute(
            AttributeFormat::RegExp("[1-9])|(1[0-3])".to_string()),
            ActionBehaviour::Edit,
        );
        assert!(check_value(&a, "6").is_err());
    }
}
