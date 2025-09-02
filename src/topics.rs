use std::path::Path;

use crate::errors::CanPiAppError;
use crate::state::{Menu, MenuItems, Topic};
use canpi_config::*;

pub fn convert_package_to_topic(pkg: &Package, title: &String) -> Result<Topic, CanPiAppError> {
    let ini_path = pkg.cfg_path.clone() + "/" + pkg.ini_file.as_str();
    if Path::new(&ini_path).is_file() {
        let json_path = pkg.cfg_path.clone() + "/" + pkg.json_file.as_str();
        if Path::new(&json_path).is_file() {
            let cfg = Cfg::new(ini_path.clone(), json_path);
            let topic = Topic {
                title: title.clone(),
                ini_file_path: ini_path,
                attr_defn: cfg,
            };
            Ok(topic)
        } else {
            Err(CanPiAppError::NotFound(format!(
                "Json file '{json_path}' not found"
            )))
        }
    } else {
        Err(CanPiAppError::NotFound(format!(
            "Configuration file '{ini_path}' not found"
        )))
    }
}

pub fn build_main_menu(package_hash: &PackageHash) -> MenuItems {
    let mut menu_items: MenuItems = Vec::new();
    if package_hash.is_empty() {
        log::warn!("No packages defined in configuration");
    } else {
        // Create the menu items from the package definitions
        for (scope, pkg) in package_hash.iter() {
            let item = Menu {
                scope: scope.to_string(),
                prompt: match &pkg.title {
                    Some(t) => t.clone(),
                    None => scope.to_string(),
                },
            };
            menu_items.push(item);
        }
        log::info!("Main menu created with {} items", menu_items.len());
    }
    menu_items
}

pub fn build_topic_menu(topic: &Topic) -> MenuItems {
    let mut menu_items: MenuItems = Vec::new();
    let item_display = Menu {
        scope: "display".to_string(),
        prompt: "Display".to_string(),
    };
    menu_items.push(item_display);
    let item_save = Menu {
        scope: "save".to_string(),
        prompt: "Save".to_string(),
    };
    menu_items.push(item_save);
    log::info!(
        "Topic menu created for '{}' with {} items",
        topic.title,
        menu_items.len()
    );
    menu_items
}

#[cfg(test)]
mod tests {
    use super::*;

    const CFG_FULL_DATA: &str = r#"
        {
            "AutoHotSpot" : {
                "cfg_path" : "${workspaceFolder}/scratch",
                "ini_file" : "hotspot_example.cfg",
                "json_file" : "hotspot_example.json"
            },
            "CANPiServer" : {
                "cfg_path" : "${workspaceFolder}/scratch",
                "ini_file" : "canpi_example.cfg",
                "json_file" : "canpi_example.json"
            }
        }"#;

    const CFG_BAD_DATA_1: &str = r#"
        {
            "AutoHotSpot" : {
                "cfg_path" : "${workspaceFolder}/scratch",
                "ini_file" : "hotspot_example.cfg",
                "json_file" : "hotspot.json"
            }
        }"#;

    const CFG_BAD_DATA_2: &str = r#"
        {
            "AutoHotSpot" : {
                "cfg_path" : "${workspaceFolder}/scratch",
                "ini_file" : "hotspot.cfg",
                "json_file" : "hotspot_example.json"
            }
        }"#;

    const CFG_BAD_DATA_3: &str = r#"
        {
            "AutoHotSpot" : {
                "cfg_path" : "/Users/thornbem/RustroverProjects/canpi-web-app-ssr/scratch",
                "ini_file" : "hotspot_example.cfg",
                "json_file" : "hotspot_example.json"
            }
        }"#;

    const CFG_PARTIAL_DATA: &str = r#"
        {
            "CANPiServer" : {
                "cfg_path" : "/home/thornbem/Work/canpi-web-app-ssr/scratch",
                "ini_file" : "canpi_example.cfg",
                "json_file" : "canpi.json"
            }
        }"#;

    // Common functions
    fn setup_file<P: AsRef<Path>>(test_file: P, data: &str) {
        let mut f = File::create(&test_file)
            .expect(format!("file creation failed '{:#?}'", test_file.as_ref().to_str()).as_str());
        f.write_all(data.as_bytes()).expect("file write failed");
    }

    fn teardown_file<P: AsRef<Path>>(test_file: P) {
        std::fs::remove_file(test_file).expect("file deletion failed");
    }

    #[test]
    fn check_html_file_name() {
        let file_name_root = Path::new("templates");
        let format_file = file_name_root;
        let mut format_file = format_file.join("top_menu.format");
        let html_file = file_name_root;
        let html_file = html_file.join("top_menu.html");
        format_file.set_extension("html");
        assert_eq!(format_file, html_file);
    }

    #[test]
    #[ignore = "reason: requires environment variable CPSSR_HOME to be set"]
    fn cfg_bad_data_3() {
        let cfg_file = "scratch/bad_data_3.json";
        setup_file(&cfg_file, CFG_BAD_DATA_3);
        let pkg_defn = Pkg::new(&cfg_file);
        teardown_file(&cfg_file);
        let packages = pkg_defn.packages;
        match packages {
            Some(pkgs) => {
                assert!(pkgs.is_empty())
            }
            None => assert!(true),
        }
    }
}
