use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use strfmt::strfmt;

use crate::errors::CanPiAppError;
use canpi_config::Pkg;

fn create_html_file<P: AsRef<Path>>(format_file: P) -> std::io::Result<File> {
    let stem = format_file
        .as_ref()
        .file_stem()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "File has no stem"))?;
    let mut new_name = OsString::from(stem);
    new_name.push(".html");
    File::create(new_name)
}

pub fn build_top_menu_html<P: AsRef<Path>>(
    pkg_defn: &Pkg,
    format_file: P,
) -> Result<(), CanPiAppError> {
    let mut format_defn = String::new();
    let mut file = File::open(format_file.as_ref())?;
    file.read_to_string(&mut format_defn)?;
    let mut html_file = create_html_file(format_file)?;
    if let Some(pkgs) = &pkg_defn.packages {
        if pkgs.is_empty() {
            html_file.write_all(b"No maintainable packages configured\n")?;
        } else {
            let mut html_code = String::new();
            for title in pkgs.keys() {
                let mut vars = HashMap::new();
                vars.insert("title".to_string(), title.clone());
                let hc = strfmt(&format_defn, &vars)?;
                html_code.push_str(&hc);
            }
            html_file.write_all(&html_code.into_bytes())?;
        }
    } else {
        html_file.write_all(b"No maintainable packages available\n")?;
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
