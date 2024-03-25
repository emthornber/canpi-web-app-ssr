use crate::errors::CanPiAppError;
use std::path::Path;

use canpi_config::Pkg;

#[macro_export]
macro_rules! pkg_name {
    () => {
        env!("CARGO_BIN_NAME")
    };
}

//const CANPI_SSR_DIR: &str = pkg_name!();
//const CANPI_SSR_DIR: &str = env!("CARGO_PKG_NAME");
const CFGFILE: &str = "/canpi-ssr.json";
const STATIC: &str = "/static";
const TEMPLATE: &str = "/templates/**/*";

/// Structure that holds configuration items expanded from EVs and static text
pub struct CanpiConfig {
    pub config_path: Option<String>,
    pub host_port: Option<String>,
    pub static_path: Option<String>,
    pub template_path: Option<String>,
    pub pkg_defn: Option<Pkg>,
}

impl CanpiConfig {
    /// Creates a new instance of the structure
    ///
    /// The contents of the EV CPSSR_HOME is used with the static text above to create path strings
    /// for items.
    ///
    /// If CPSSR_HOME is not defined or does not point to a valid directory then an error result is
    /// returned.
    ///
    /// If the EVs CFGFILE is not defined or does not point to a valid file
    /// then the entries config_file and svc_defn in the struct are set to None.
    ///
    /// If the EV HOST_PORT is not defined then the entry in the struct is set to None.  No further
    /// validation is done if the EV does exist.
    ///
    pub fn new() -> Result<CanpiConfig, CanPiAppError> {
        let h = std::env::var("CPSSR_HOME");
        match h {
            Ok(home) => {
                let cps_home = home;
                if !Path::new(&cps_home).is_dir() {
                    return Err(CanPiAppError::NotFound(
                        "EV CPSSR_HOME not a directory".to_string(),
                    ));
                }
                let mut cfg = CanpiConfig {
                    config_path: None,
                    host_port: None,
                    static_path: None,
                    template_path: None,
                    pkg_defn: None,
                };

                let cfile = cps_home.clone() + "/" + STATIC + CFGFILE;
                if Path::new(&cfile).is_file() {
                    cfg.config_path = Some(cfile.clone());
                    let mut pkg = Pkg::new();
                    pkg.load_packages(cfile)
                        .expect("Cannot load package configurations");
                    cfg.pkg_defn = Some(pkg);
                } else {
                    return Err(CanPiAppError::NotFound(format!(
                        "Configuration file '{cfile}' not found"
                    )));
                }

                if let Ok(port) = std::env::var("HOST_PORT") {
                    cfg.host_port = Some(port);
                } else {
                    return Err(CanPiAppError::NotFound(
                        "EV HOST_PORT not valid".to_string(),
                    ));
                }
                let sdir = cps_home.clone() + "/" + STATIC;
                if Path::new(&sdir).is_dir() {
                    cfg.static_path = Some(sdir);
                }
                let tdir = cps_home.clone() + "/" + TEMPLATE;
                let grandparent = Path::new(&tdir).parent().unwrap().parent().unwrap();
                if grandparent.is_dir() {
                    cfg.template_path = Some(tdir);
                }

                return Ok(cfg);
            }
            _ => {}
        }
        Err(CanPiAppError::NotFound(
            "EV CPSSR_HOME not defined".to_string(),
        ))
    }
}
