use crate::errors::CanPiAppError;
use canpi_config::{Cfg, PackageHash};

/// Definition of Attributes for a Topic
pub struct Topic {
    pub title: String,
    pub ini_file_path: String,
    pub attr_defn: Cfg,
    /// Optional service name without the .service suffix
    /// This is used to check if the service exists in the systemd directory.
    /// If the service exists, it will be stored here; otherwise, it will be None
    /// This is used to determine if the topic can be restarted.
    pub service_name: Option<String>,
}

impl Topic {
    pub fn restart_topic(&self) -> Result<(), CanPiAppError> {
        // Logic to restart the topic service
        if let Some(service_name) = &self.service_name {
            // Restart logic here, e.g., using systemctl
            // For now, we just return Ok to simulate success
            Ok(())
        } else {
            Err(CanPiAppError::NotFound(
                "Service name not found for topic".to_string(),
            ))
        }
    }
}
pub struct AppState {
    pub layout_name: String,
    pub project_id: String,
    pub template_root: String,
    pub current_topic: Option<Topic>,
    pub packages: PackageHash,
}
