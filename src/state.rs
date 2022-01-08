use std::ffi::OsString;
use ini::Ini;

pub struct AppState {
    pub layout_name: String,
    pub canpi_cfg: Ini,
}