use itertools::Itertools;
use log;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::errors::CanPiAppError;
use crate::state::{Topic, TopicHash};
use canpi_config::*;

pub fn convert_package_to_topic(pkg: &Package) -> Result<Topic, CanPiAppError> {
    let ini_path = pkg.cfg_path.clone() + "/" + pkg.ini_file.as_str();
    if Path::new(&ini_path).is_file() {
        let json_path = pkg.cfg_path.clone() + "/" + pkg.json_file.as_str();
        if Path::new(&json_path).is_file() {
            let mut cfg = Cfg::new();
            let _ = cfg.load_configuration(ini_path.clone(), json_path);
            let topic = Topic {
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

pub fn load_pkg_cfgs(pkg_defn: &Pkg) -> TopicHash {
    let mut topics = TopicHash::new();
    if let Some(pkg_hash) = &pkg_defn.packages {
        for (k, v) in pkg_hash.iter() {
            if let Ok(attr) = convert_package_to_topic(v) {
                topics.insert(k.to_string(), attr);
            }
        }
    }
    if topics.is_empty() {
        log::warn!("No package attribute definitions found");
    }
    topics
}

fn create_html_file<P: AsRef<Path>>(format_file: P) -> std::io::Result<File> {
    let mut html_file = PathBuf::from(format_file.as_ref());
    html_file.set_extension("html");
    File::create(html_file)
}

pub fn build_top_menu_html<P: AsRef<Path>>(
    topic_hash: &TopicHash,
    format_file: P,
) -> Result<(), CanPiAppError> {
    let mut format_defn = String::new();
    let mut file = File::open(format_file.as_ref())?;
    file.read_to_string(&mut format_defn)?;
    let mut html_file = create_html_file(format_file)?;
    if topic_hash.is_empty() {
        html_file.write_all(b"No maintainable packages configured\n")?;
    } else {
        let mut html_code = String::new();
        for title in topic_hash.keys().sorted() {
            let line = format_defn.as_str().replace("|title|", title.as_str());
            html_code.push_str(line.as_str());
        }
        html_file.write_all(&html_code.into_bytes())?;
    }

    Ok(())
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
                "cfg_path" : "/Users/thornbem/RustroverProjects/canpi-web-app-ssr/SCRATCH",
                "ini_file" : "hotspot_example.cfg",
                "json_file" : "hotspot_example.json"
            }
        }"#;

    const CFG_PARTIAL_DATA: &str = r#"
        {
            "CANPiServer" : {
                "cfg_path" : "/Users/thornbem/RustroverProjects/canpi-web-app-ssr/scratch",
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
        let file_name_root =
            Path::new("/Users/thornbem/RustroverProjects/canpi_web_app_ssr/scratch");
        let mut format_file = file_name_root;
        let mut format_file = format_file.join("top_menu.format");
        let mut html_file = file_name_root;
        let mut html_file = html_file.join("top_menu.html");
        format_file.set_extension("html");
        assert_eq!(format_file, html_file);
    }

    #[test]
    fn cfg_bad_data_3() {
        let cfg_file =
            "/Users/thornbem/RustroverProjects/canpi-web-app-ssr/scratch/bad_data_3.json";
        setup_file(&cfg_file, CFG_BAD_DATA_3);
        let mut pkg_defn = Pkg::new();
        pkg_defn
            .load_packages(&cfg_file)
            .expect("No files found - duff directory");
        build_top_menu_html(
            &pkg_defn,
            "/Users/thornbem/RustroverProjects/canpi-web-app-ssr/templates/top_menu.format",
        )
        .expect("no called");
        teardown_file(&cfg_file);
    }
}
