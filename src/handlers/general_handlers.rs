use actix_web::{web, Error, HttpResponse, Result};
use std::sync::Mutex;

use crate::state::AppState;
use crate::errors::CanPiAppError;

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
