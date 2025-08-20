use itertools::Itertools;
// use log;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::errors::CanPiAppError;
use crate::state::Topic;
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

// pub fn load_pkg_cfgs(pkg_defn: &Pkg) -> TopicHash {
//     let mut topics = TopicHash::new();
//     if let Some(pkg_hash) = &pkg_defn.packages {
//         for (k, v) in pkg_hash.iter() {
//             if let Ok(attr) = convert_package_to_topic(v) {
//                 topics.insert(k.to_string(), attr);
//             }
//         }
//     }
//     if topics.is_empty() {
//         log::warn!("No package attribute definitions found");
//     }
//     topics
// }

fn create_html_file<P: AsRef<Path>>(format_file: P) -> std::io::Result<File> {
    let mut html_file = PathBuf::from(format_file.as_ref());
    html_file.set_extension("html");
    File::create(html_file)
}

pub fn build_top_menu_html<P: AsRef<Path>>(
    package_hash: &PackageHash,
    format_file: P,
) -> Result<(), CanPiAppError> {
    let mut format_defn = String::new();
    let mut file = File::open(format_file.as_ref())?;
    file.read_to_string(&mut format_defn)?;
    let mut html_file = create_html_file(format_file)?;
    if package_hash.is_empty() {
        html_file.write_all(b"<li><br>No maintainable packages configured<br></li>")?;
    } else {
        let mut html_code = String::new();
        for title in package_hash.keys().sorted() {
            let line = format_defn.as_str().replace("|title|", title.as_str());
            html_code.push_str(line.as_str());
        }
        html_file.write_all(&html_code.into_bytes())?;
    }

    Ok(())
}

pub fn build_topic_menu_html<P: AsRef<Path>>(
    topic: &Topic,
    format_file: P,
) -> Result<(), CanPiAppError> {
    let mut format_defn = String::new();
    let mut file = File::open(format_file.as_ref())?;
    file.read_to_string(&mut format_defn)?;
    let mut html_file = create_html_file(format_file)?;
    let mut visibility = "<li hidden>";
    if let Some(_name) = topic.service_name.clone() {
        visibility = "<li>";
    }
    let mut html_code = String::new();
    let line = format_defn.as_str().replace("|visibility|", visibility);
    html_code.push_str(line.as_str());
    html_file.write_all(&html_code.into_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn service_name_exists() {
        let service_name = Some("systemd-halt".to_string());
        let svc_name = check_service_name(service_name);
        assert!(svc_name.is_some());
        assert_eq!(svc_name.unwrap(), "systemd-halt");
    }

    #[test]
    fn service_name_does_not_exist() {
        let service_name = Some("nonexistent-service".to_string());
        let svc_name = check_service_name(service_name);
        assert!(svc_name.is_none());
    }

    #[test]
    fn service_name_none() {
        let service_name: Option<String> = None;
        let svc_name = check_service_name(service_name);
        assert!(svc_name.is_none());
    }

    // Test converting package to topic
    // This will check that the ini_file_path and service_name are set correctly
    #[test]
    fn cfg_full_data_1() {
        let packages = setup_pkgs(CFG_FULL_DATA);
        assert!(packages.is_some());
        let packages = packages.unwrap();
        assert_eq!(packages.len(), 2);
        let topic = convert_package_to_topic(&packages["CANPiServer"], &"CANPiServer".to_string());
        assert!(topic.is_ok());
        let topic = topic.unwrap();
        assert_eq!(topic.title, "CANPiServer");
        assert_eq!(topic.ini_file_path, "scratch/canpi_example.cfg");
        assert!(topic.service_name.is_some());
        assert_eq!(topic.service_name.unwrap(), "ssh");
    }

    // This will check that the missing service_name is 'None'
    #[test]
    fn cfg_full_data_2() {
        let packages = setup_pkgs(CFG_FULL_DATA);
        assert!(packages.is_some());
        let packages = packages.unwrap();
        assert_eq!(packages.len(), 2);
        let topic = convert_package_to_topic(&packages["AutoHotSpot"], &"AutoHotSpot".to_string());
        assert!(topic.is_ok());
        let topic = topic.unwrap();
        assert_eq!(topic.title, "AutoHotSpot");
        assert_eq!(topic.ini_file_path, "scratch/hotspot_example.cfg");
        assert!(topic.service_name.is_none());
    }

    #[test]
    fn cfg_bad_data_1() {
        let packages = setup_pkgs(CFG_BAD_DATA_1);
        assert!(packages.is_some());
        let packages = packages.unwrap();
        assert_eq!(packages.len(), 1);
        let topic = convert_package_to_topic(&packages["AutoHotSpot"], &"AutoHotSpot".to_string());
        assert!(topic.is_err());
    }

    #[test]
    fn cfg_bad_data_2() {
        let packages = setup_pkgs(CFG_BAD_DATA_2);
        assert!(packages.is_some());
        let packages = packages.unwrap();
        assert_eq!(packages.len(), 1);
        let topic = convert_package_to_topic(&packages["AutoHotSpot"], &"AutoHotSpot".to_string());
        assert!(topic.is_err());
    }

    #[test]
    fn cfg_bad_data_3() {
        let packages = setup_pkgs(CFG_BAD_DATA_3);
        assert!(packages.is_some());
        let packages = packages.unwrap();
        assert_eq!(packages.len(), 1);
        let topic = convert_package_to_topic(&packages["AutoHotSpot"], &"AutoHotSpot".to_string());
        assert!(topic.is_err());
    }

    #[test]
    fn visibility_yes_test() {
        let packages = setup_pkgs(CFG_FULL_DATA);
        assert!(packages.is_some());
        let packages = packages.unwrap();
        assert_eq!(packages.len(), 2);
        let topic = convert_package_to_topic(&packages["CANPiServer"], &"CANPiServer".to_string());
        assert!(topic.is_ok());
        let topic = topic.unwrap();
        let format_file = PathBuf::from("templates/topic_menu.format");
        let result = build_topic_menu_html(&topic, format_file);
        assert!(result.is_ok());
        let mut html_defn = String::new();
        let mut file = File::open(Path::new("templates/topic_menu.html")).unwrap();
        file.read_to_string(&mut html_defn).unwrap();
        println!("HTML Definition: {}", html_defn);
        assert!(html_defn.contains("<li>"));
    }

    #[test]
    fn visibility_no_test() {
        let packages = setup_pkgs(CFG_FULL_DATA);
        assert!(packages.is_some());
        let packages = packages.unwrap();
        assert_eq!(packages.len(), 2);
        let topic = convert_package_to_topic(&packages["AutoHotSpot"], &"AutoHotSpot".to_string());
        assert!(topic.is_ok());
        let topic = topic.unwrap();
        let format_file = PathBuf::from("templates/topic_menu.format");
        let result = build_topic_menu_html(&topic, format_file);
        assert!(result.is_ok());
        let mut html_defn = String::new();
        let mut file = File::open(Path::new("templates/topic_menu.html")).unwrap();
        file.read_to_string(&mut html_defn).unwrap();
        println!("HTML Definition: {}", html_defn);
        assert!(html_defn.contains("<li hidden>"));
    }
}
