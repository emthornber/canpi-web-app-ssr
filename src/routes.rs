use actix_files as fs;
use crate::handlers::autohs_handlers::*;
use crate::handlers::canpi_handlers::*;
use actix_web::web;

pub fn canpi_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/canpi")
            .service(web::resource("")
                .route(web::get().to(status_handler)))
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
        web::scope("/autohs")
            .service(web::resource("")
                .route(web::get().to(status_handler)))
            .service(web::resource("/display")
                .route(web::get().to(display_autohs)))
            .service(web::resource("/edit")
                .route(web::get().to(edit_autohs)))
            .service(web::resource("/update")
                .route(web::post().to(update_autohs)))
    );
}