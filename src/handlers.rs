use crate::state::AppState;
use crate::errors::CanPiAppError;
use actix_web::{web, Error, HttpResponse, Result};

pub async fn status_handler(
    app_state: web::Data<AppState>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("layout_name", &app_state.layout_name);
    let s = tmpl
        .render("index.html", &ctx)
        .map_err(|_| CanPiAppError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn new_config(
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new config");
    let response = format!("blah blah blah");
    HttpResponse::Ok().json(&response)
}

#[cfg(test)]
mod tests {
}