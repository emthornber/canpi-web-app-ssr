use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CanPiConfig {
    pub canid: String,
    pub node_number: String,
    pub start_event_id: String,
    pub node_mode: String,
    pub can_grid: String,
    pub edserver: String,
    pub candevice: String,
    pub cangrid_port: String,
    pub create_log_file: String,
    pub fn_momentary: String,
    pub button_pin: String,
    pub green_led_pin: String,
    pub yellow_led_pin: String,
    pub red_led_pin: String,
    pub logappend: String,
    pub logfile: String,
    pub loglevel: String,
    pub service_name: String,
    pub tcpport: String,
    pub turnout_file: String,
    pub shutdown_code: String,
    pub orphan_timeout: String,
    pub ap_ssid: String,
    pub ap_passwd: String,
    pub ap_channel: String,
    pub router_ssid: String,
    pub router_passwd: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCanPiConfig {
    pub start_event_id: Option<String>,
    pub cangrid_port: Option<String>,
    pub loglevel: Option<String>,
    pub tcpport: Option<String>,
    pub ap_ssid: Option<String>,
    pub ap_channel: Option<String>,
    pub router_ssid: Option<String>,
}

impl From<web::Json<UpdateCanPiConfig>> for UpdateCanPiConfig {
    fn from(update_config: web::Json<UpdateCanPiConfig>) -> Self {
        UpdateCanPiConfig {
            start_event_id: update_config.start_event_id.clone(),
            cangrid_port: update_config.cangrid_port.clone(),
            loglevel: update_config.loglevel.clone(),
            tcpport: update_config.tcpport.clone(),
            ap_ssid: update_config.ap_ssid.clone(),
            ap_channel: update_config.ap_channel.clone(),
            router_ssid: update_config.router_ssid.clone(),
        }
    }
}