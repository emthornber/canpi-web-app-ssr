use canpi_config::{Cfg, PackageHash};

/// Definition of Attributes for a Topic
pub struct Topic {
    pub title: String,
    pub ini_file_path: String,
    pub attr_defn: Cfg,
}

pub struct AppState {
    pub layout_name: String,
    pub project_id: String,
    pub current_topic: Option<Topic>,
    pub packages: PackageHash,
}
