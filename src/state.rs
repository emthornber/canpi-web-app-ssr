use canpi_config::{Cfg, PackageHash};

/// Menu Structure
#[derive(serde::Serialize)]
pub struct Menu {
    /// Target of pressing menu item
    pub scope: String,
    /// Text to display for menu item
    pub prompt: String,
}

/// Type alias for Menu Vector
pub type MenuItems = Vec<Menu>;

/// Definition of Attributes for a Topic
#[derive(Debug, Clone)]
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
    pub main_menu: MenuItems,
    pub topic_menu: MenuItems,
}
