use std::path::Path;
use std::process::Command;

use crate::errors::CanPiAppError;
use crate::state::{Menu, MenuItems, Topic};
use canpi_config::*;

pub fn check_service_name(service_name: Option<String>) -> Option<String> {
    if let Some(name) = service_name {
        let svc_path = "/lib/systemd/system/";
        let service_file = svc_path.to_owned() + name.as_str() + ".service";
        if Path::new(&service_file).is_file() {
            log::debug!("Service '{}' exists at '{}'", name, service_file);
            Some(name.clone())
        } else {
            log::debug!("Service '{}' does not exist at '{}'", name, service_file);
            None
        }
    } else {
        log::debug!("No service name provided");
        None
    }
}

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
                service_name: check_service_name(pkg.service_name.clone()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger::Target;
    use log::LevelFilter;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    fn init_logging() {
        let _ = env_logger::builder()
            .target(Target::Stdout)
            .filter_level(LevelFilter::max())
            .is_test(true)
            .try_init();
    }

    // Test data service_name is checking a service on the dev machine
    const CFG_FULL_DATA: &str = r#"
        {
            "AutoHotSpot" : {
                "cfg_path" : "scratch",
                "ini_file" : "hotspot_example.cfg",
                "json_file" : "hotspot_example.json"
            },
            "CANPiServer" : {
                "cfg_path" : "scratch",
                "ini_file" : "canpi_example.cfg",
                "json_file" : "canpi_example.json",
                "service_name" : "ssh"
            }
        }"#;

    // Test data with bad json_file name
    const CFG_BAD_DATA_1: &str = r#"
        {
            "AutoHotSpot" : {
                "cfg_path" : "scratch",
                "ini_file" : "hotspot_example.cfg",
                "json_file" : "hotspot.json"
            }
        }"#;

    // Test data with bad ini_file name
    const CFG_BAD_DATA_2: &str = r#"
        {
            "AutoHotSpot" : {
                "cfg_path" : "scratch",
                "ini_file" : "hotspot.cfg",
                "json_file" : "hotspot_example.json"
            }
        }"#;

    // Test data with bad cfg_path name
    const CFG_BAD_DATA_3: &str = r#"
        {
            "AutoHotSpot" : {
                "cfg_path" : "/Users/thornbem/RustroverProjects/canpi-web-app-ssr/scratch",
                "ini_file" : "hotspot_example.cfg",
                "json_file" : "hotspot_example.json"
            }
        }"#;

    // Common functions
    fn setup_file<P: AsRef<Path>>(test_file: P, data: &str) {
        let mut f = File::create(&test_file)
            .expect(format!("file creation failed '{:#?}'", test_file.as_ref().to_str()).as_str());
        f.write_all(data.as_bytes()).expect("file write failed");
    }

    fn setup_pkgs(data: &str) -> Option<PackageHash> {
        let cfg_file = "scratch/pkg_data.json";
        setup_file(&cfg_file, data);
        let pkg_defn = Pkg::new(&cfg_file);
        teardown_file(&cfg_file);
        pkg_defn.packages
    }

    fn teardown_file<P: AsRef<Path>>(test_file: P) {
        std::fs::remove_file(test_file).expect("file deletion failed");
    }

    #[test]
    fn service_name_exists() {
        // Initialise Logger
        init_logging();

        let service_name = Some("systemd-halt".to_string());
        let svc_name = check_service_name(service_name);
        assert!(svc_name.is_some());
        assert_eq!(svc_name.unwrap(), "systemd-halt");
    }

    #[test]
    fn service_name_does_not_exist() {
        // Initialise Logger
        init_logging();

        let service_name = Some("nonexistent-service".to_string());
        let svc_name = check_service_name(service_name);
        assert!(svc_name.is_none());
    }

    #[test]
    fn service_name_none() {
        // Initialise Logger
        init_logging();

        let service_name: Option<String> = None;
        let svc_name = check_service_name(service_name);
        assert!(svc_name.is_none());
    }

    // Test converting package to topic
    // This will check that the ini_file_path and service_name are set correctly
    #[test]
    fn cfg_full_data_1() {
        // Initialise Logger
        init_logging();

        let packages = setup_pkgs(CFG_FULL_DATA);
        assert!(packages.is_some());
        let packages = packages.unwrap();
        assert_eq!(packages.len(), 2);
        let topic = convert_package_to_topic(&packages["canpiserver"], &"CANPiServer".to_string());
        assert!(topic.is_ok());
        let topic = topic.unwrap();
        assert_eq!(topic.title, "CANPiServer");
        assert_eq!(topic.ini_file_path, "scratch/canpi_example.cfg");
        assert!(topic.service_name.is_some());
        let menu = build_topic_menu(&topic);
        assert_eq!(menu.len(), 3);
        assert_eq!(topic.service_name.unwrap(), "ssh");
    }

    // This will check that the missing service_name is 'None'
    #[test]
    fn cfg_full_data_2() {
        // Initialise Logger
        init_logging();

        let packages = setup_pkgs(CFG_FULL_DATA);
        assert!(packages.is_some());
        let packages = packages.unwrap();
        assert_eq!(packages.len(), 2);
        let topic = convert_package_to_topic(&packages["autohotspot"], &"AutoHotSpot".to_string());
        assert!(topic.is_ok());
        let topic = topic.unwrap();
        assert_eq!(topic.title, "AutoHotSpot");
        assert_eq!(topic.ini_file_path, "scratch/hotspot_example.cfg");
        assert!(topic.service_name.is_none());
        let menu = build_topic_menu(&topic);
        assert_eq!(menu.len(), 2);
    }

    #[test]
    fn cfg_bad_data_1() {
        // Initialise Logger
        init_logging();

        let packages = setup_pkgs(CFG_BAD_DATA_1);
        assert!(packages.is_some());
        let packages = packages.unwrap();
        assert_eq!(packages.len(), 1);
        let topic = convert_package_to_topic(&packages["autohotspot"], &"AutoHotSpot".to_string());
        assert!(topic.is_err());
    }

    #[test]
    fn cfg_bad_data_2() {
        // Initialise Logger
        init_logging();

        let packages = setup_pkgs(CFG_BAD_DATA_2);
        assert!(packages.is_some());
        let packages = packages.unwrap();
        assert_eq!(packages.len(), 1);
        let topic = convert_package_to_topic(&packages["autohotspot"], &"AutoHotSpot".to_string());
        assert!(topic.is_err());
    }

    #[test]
    fn cfg_bad_data_3() {
        // Initialise Logger
        init_logging();

        let packages = setup_pkgs(CFG_BAD_DATA_3);
        assert!(packages.is_some());
        let packages = packages.unwrap();
        assert_eq!(packages.len(), 1);
        let topic = convert_package_to_topic(&packages["autohotspot"], &"AutoHotSpot".to_string());
        assert!(topic.is_err());
    }
}
