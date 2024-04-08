use crate::handlers::general_handlers::*;
use crate::handlers::topic_handlers::*;
use actix_web::web;
use lazy_static::lazy_static;
use std::collections::HashMap;

const LAYOUT: &str = "/layout";
const CONFIRM: &str = "/confirm";
const DISPLAY: &str = "/display";
const EDIT: &str = "/edit";
const PKG: &str = "/pkg";
const SAVE: &str = "/save";
const TITLE: &str = "/pkg/{title}";
const TOPIC: &str = "/topic";
const UPDATE: &str = "/update";

lazy_static! {
    pub static ref ROUTE_DATA: HashMap<&'static str, String> = {
        let mut map = HashMap::new();

        map.insert("root", format!("{LAYOUT}"));
        map.insert("confirm", format!("{LAYOUT}{TOPIC}{CONFIRM}"));
        map.insert("display", format!("{LAYOUT}{TOPIC}{DISPLAY}"));
        map.insert("edit", format!("{LAYOUT}{TOPIC}{EDIT}"));
        map.insert("pkg", format!("{LAYOUT}{PKG}"));
        map.insert("save", format!("{LAYOUT}{TOPIC}{SAVE}"));
        map.insert("topic", format!("{LAYOUT}{TOPIC}"));
        map.insert("update", format!("{LAYOUT}{TOPIC}{UPDATE}"));

        map
    };
}

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ROUTE_DATA["root"].as_str())
            .service(web::resource("").route(web::get().to(status_handler)))
            .service(web::resource(TITLE).route(web::get().to(status_pkg))),
    );
}

pub fn topic_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ROUTE_DATA["topic"].as_str())
            .service(web::resource("").route(web::get().to(status_topic)))
            .service(web::resource(DISPLAY).route(web::get().to(display_topic)))
            .service(web::resource(EDIT).route(web::get().to(edit_topic)))
            .service(web::resource(SAVE).route(web::get().to(save_topic)))
            .service(web::resource(UPDATE).route(web::post().to(update_topic))),
    );
}
