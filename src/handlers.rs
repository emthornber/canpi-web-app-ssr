use super::state::AppState;
use super::models::WiFiParameters;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use std::fmt::format;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_config(
    new_config: web::Json<WiFiParameters>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new config");
    let mut wifi_params = app_state.wifi_params.lock().unwrap();
    *wifi_params = WiFiParameters {
        ap_ssid: new_config.ap_ssid.clone(),
        ap_passwd: new_config.ap_passwd.clone(),
        ap_channel: new_config.ap_channel.clone(),
        router_ssid: new_config.router_ssid.clone(),
        router_passwd: new_config.router_passwd.clone(),
    };
    let response = format!("{} {} {}", wifi_params.ap_ssid, wifi_params.ap_channel, wifi_params.router_ssid);
    HttpResponse::Ok().json(&response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_config_test() {
        let config = web::Json(WiFiParameters {
            ap_ssid: "aUnitTest".into(),
            ap_passwd: "aBlank".into(),
            ap_channel: "6".into(),
            router_ssid: "rUnitTest".into(),
            router_passwd: "rBlank".into(),
        });
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            wifi_params: Mutex::new(WiFiParameters {
                ap_ssid: "".into(),
                ap_passwd: "".into(),
                ap_channel: "".into(),
                router_ssid: "".into(),
                router_passwd: "".into(),
            }),
        });
        let resp = new_config(config, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}