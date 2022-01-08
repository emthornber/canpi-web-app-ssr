use std::ffi::OsString;
use std::sync::Mutex;
use ini::Ini;

pub struct AppState {
    pub layout_name: Mutex<OsString>,
    pub canpi_cfg: Mutex<Ini>,
}