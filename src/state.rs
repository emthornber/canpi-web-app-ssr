use canpi_config::Cfg;
use std::collections::HashMap;

/// Definition of Attributes for a Topic
pub struct Topic {
    pub ini_file_path: String,
    pub attr_defn: Cfg,
}

/// Type alias based on a HashMap for a set of Packages
pub type TopicHash = HashMap<String, Topic>;

pub struct AppState {
    pub layout_name: String,
    pub project_id: String,
    pub current_topic: Option<String>,
    pub topics: TopicHash,
}
