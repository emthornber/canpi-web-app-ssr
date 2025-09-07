use crate::topic::Topic;
use canpi_config::PackageHash;
use serde::Serialize;

/// Menu Structure
#[derive(Serialize)]
pub struct Menu {
    /// Scope of target, e.g., "pkg"
    pub scope: String,
    /// Target of the menu item
    pub target: String,
    /// Text to display for menu item
    pub prompt: String,
}

/// Type alias for Menu Vector
pub type MenuItems = Vec<Menu>;

pub fn build_main_menu(package_hash: &PackageHash) -> MenuItems {
    let mut menu_items: MenuItems = Vec::new();
    if package_hash.is_empty() {
        log::warn!("No packages defined in configuration");
    } else {
        // Create the menu items from the package definitions
        for (target, pkg) in package_hash.iter() {
            let item = Menu {
                scope: "pkg".to_string(),
                target: target.to_string(),
                prompt: match &pkg.title {
                    Some(t) => t.clone(),
                    None => target.to_string(),
                },
            };
            menu_items.push(item);
        }
        log::info!("Main menu created with {} items", menu_items.len());
    }
    menu_items
}

const TOPIC_MENU_ITEMS: [(&str, &str); 3] = [
    ("display", "Display"),
    ("save", "Save"),
    ("restart", "Restart"),
];

pub fn build_topic_menu(topic: &Topic) -> MenuItems {
    let mut menu_items: MenuItems = Vec::new();
    for (target, prompt) in TOPIC_MENU_ITEMS.iter() {
        // If the topic service_name is undefined then skip the restart option
        if *target == "restart" {
            if topic.service_name.is_none() {
                continue;
            }
        }
        let item = Menu {
            scope: "topic".to_string(),
            target: target.to_string(),
            prompt: prompt.to_string(),
        };
        menu_items.push(item);
    }
    log::info!(
        "Topic menu created for '{}' with {} items",
        topic.title,
        menu_items.len()
    );
    menu_items
}

pub struct AppState {
    pub layout_name: String,
    pub project_id: String,
    pub current_topic: Option<Topic>,
    pub packages: PackageHash,
    pub main_menu: MenuItems,
    pub topic_menu: MenuItems,
}
