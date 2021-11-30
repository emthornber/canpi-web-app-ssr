use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WiFiParameters {
    pub ap_ssid: String,
    pub ap_passwd: String,
    pub ap_channel: String,
    pub router_ssid: String,
    pub router_passwd: String,
}

impl From<web::Json<WiFiParameters>> for WiFiParameters {
    fn from(wifiparams: web::Json<WiFiParameters>) -> Self {
        WiFiParameters {
            ap_ssid: wifiparams.ap_ssid.clone(),
            ap_passwd: wifiparams.ap_passwd.clone(),
            ap_channel: wifiparams.ap_channel.clone(),
            router_ssid: wifiparams.router_ssid.clone(),
            router_passwd: wifiparams.router_passwd.clone(),
        }
    }
}