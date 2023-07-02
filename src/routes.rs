use crate::handlers::autohs_handlers::*;
use crate::handlers::canpi_handlers::*;
use crate::handlers::general_handlers::*;
use actix_web::web;
use lazy_static::lazy_static;
use std::collections::HashMap;

const AUTOHS: &str = "/autohs";
const LAYOUT: &str = "/layout";
const CANPI: &str = "/canpi";
const CONFIRM: &str = "/confirm";
const DISPLAY: &str = "/display";
const EDIT: &str = "/edit";
const SAVE: &str = "/save";
const UPDATE: &str = "/update";

lazy_static! {
    pub static ref ROUTE_DATA: HashMap<&'static str, String> = {
        let mut map = HashMap::new();
        map.insert("root", format!("{LAYOUT}"));
        map.insert("canpi", format!("{LAYOUT}{CANPI}"));
        map.insert("cconfirm", format!("{LAYOUT}{CANPI}{CONFIRM}"));
        map.insert("cdisplay", format!("{LAYOUT}{CANPI}{DISPLAY}"));
        map.insert("cedit", format!("{LAYOUT}{CANPI}{EDIT}"));
        map.insert("csave", format!("{LAYOUT}{CANPI}{SAVE}"));
        map.insert("cupdate", format!("{LAYOUT}{CANPI}{UPDATE}"));
        map.insert("autohs", format!("{LAYOUT}{AUTOHS}"));
        map.insert("aconfirm", format!("{LAYOUT}{AUTOHS}{CONFIRM}"));
        map.insert("adisplay", format!("{LAYOUT}{AUTOHS}{DISPLAY}"));
        map.insert("aedit", format!("{LAYOUT}{AUTOHS}{EDIT}"));
        map.insert("asave", format!("{LAYOUT}{AUTOHS}{SAVE}"));
        map.insert("aupdate", format!("{LAYOUT}{AUTOHS}{UPDATE}"));
        map
    };
}

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ROUTE_DATA["root"].as_str())
            .service(web::resource("").route(web::get().to(status_handler)))
            .service(web::resource(CANPI).route(web::get().to(status_canpi)))
            .service(web::resource(AUTOHS).route(web::get().to(status_autohs))),
    );
}

pub fn canpi_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ROUTE_DATA["canpi"].as_str())
            .service(web::resource("").route(web::get().to(status_canpi)))
            .service(web::resource(DISPLAY).route(web::get().to(display_canpi)))
            .service(web::resource(EDIT).route(web::get().to(edit_canpi)))
            .service(web::resource(SAVE).route(web::get().to(save_canpi)))
            .service(web::resource(UPDATE).route(web::post().to(update_canpi))),
    );
}

pub fn autohs_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ROUTE_DATA["autohs"].as_str())
            .service(web::resource("").route(web::get().to(status_autohs)))
            .service(web::resource(DISPLAY).route(web::get().to(display_autohs)))
            .service(web::resource(EDIT).route(web::get().to(edit_autohs)))
            .service(web::resource(SAVE).route(web::get().to(save_autohs)))
            .service(web::resource(UPDATE).route(web::post().to(update_autohs))),
    );
}
