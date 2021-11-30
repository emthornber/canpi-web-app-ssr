use std::sync::Mutex;
use crate::models::WiFiParameters;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub wifi_params: Mutex<WiFiParameters>,
}