use canpi_config::Cfg;

pub struct AppState {
    pub layout_name: String,
    pub project_id: String,
    pub autohs_cfg: Cfg,
    pub canpi_cfg: Cfg,
}