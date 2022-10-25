use actix_files as fs;
use crate::handlers::autohs_handlers::*;
use crate::handlers::canpi_handlers::*;
use crate::handlers::general_handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/layout")
            .service(web::resource("")
                .route(web::get().to(status_handler)))
            .service(web::resource("/canpi")
                .route(web::get().to(status_canpi)))
            .service(web::resource("/autohs")
                .route(web::get().to(status_autohs)))
    );
}

pub fn canpi_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/layout/canpi")
            .service(web::resource("")
                .route(web::get().to(status_canpi)))
            .service(web::resource("/display")
                .route(web::get().to(display_canpi)))
            .service(web::resource("/edit")
                .route(web::get().to(edit_canpi)))
            .service(web::resource("/update")
                .route(web::post().to(update_canpi)))
    );
}

pub fn autohs_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/layout/autohs")
            .service(web::resource("")
                .route(web::get().to(status_autohs)))
            .service(web::resource("/display")
                .route(web::get().to(display_autohs)))
            .service(web::resource("/edit")
                .route(web::get().to(edit_autohs)))
            .service(web::resource("/update")
                .route(web::post().to(update_autohs)))
    );
}