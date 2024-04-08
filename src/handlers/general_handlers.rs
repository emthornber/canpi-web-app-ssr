use actix_web::{web, Error, HttpResponse, Result};
use std::sync::Mutex;

use crate::errors::CanPiAppError;
use crate::state::AppState;

use super::topic_handlers::status_topic;

pub async fn status_handler(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let app_state = app_state.lock().unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    ctx.insert("project_id", &app_state.project_id);
    let s = tmpl
        .render("index.html", &ctx)
        .map_err(|_| CanPiAppError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn status_pkg(
    app_state: web::Data<Mutex<AppState>>,
    tmpl: web::Data<tera::Tera>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    {
        let topic = path.into_inner();
        let mut app_state = app_state.lock().unwrap();
        // Assume failure and reset current topic
        app_state.current_topic = None;
        // Check that the topic is valid
        if app_state.topics.contains_key(&topic) {
            app_state.current_topic = Some(topic.clone());
        }
        // The mutex guard gets dropped here as app_state goes out of scope
    }
    // Render the topic index web page
    status_topic(app_state, tmpl).await
}
