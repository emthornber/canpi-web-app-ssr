use crate::errors::CanPiAppError;
use std::borrow::ToOwned;
use std::path::Path;
#[macro_export]
macro_rules! pkg_name {
    () => {
        env!("CARGO_BIN_NAME")
    };
}

const CANPI_SSR_DIR: &str = pkg_name!();
//const CANPI_SSR_DIR: &str = env!("CARGO_PKG_NAME");
const CFGFILE: &str = "/canpi-config-defn.json";
const STATIC: &str = "/static";
const TEMPLATE: &str = "/templates/**/*";

#[derive(Clone)]
/// Structure that holds configuration items expanded from EVs and static text
pub struct CanpiConfig {
    pub autohs_ini_path: Option<String>,
    pub canpi_ini_path: Option<String>,
    pub config_path: Option<String>,
    pub host_port: Option<String>,
    pub static_path: Option<String>,
    pub template_path: Option<String>,
}

impl CanpiConfig {
    /// Creates a new instance  of the structure
    ///
    /// The contents of the EV CPS_HOME is used with the static text above to create path strings
    /// for items.
    ///
    /// If CPS_HOME is not defined or does not point to a valid directory then an error result is
    /// returned.
    ///
    /// If the EVs AUTOHS_INI_FILE or CANPI_INI_FILE are not defined or do not point to valid files
    /// then the entries in the struct are set to None.
    ///
    /// If the EV HOST_PORT is not defined then the entry in the struct is set to None.  No further
    /// validation is done if the EV does exist.
    ///
    pub fn new() -> Result<CanpiConfig, CanPiAppError> {
        let h = std::env::var("CPS_HOME");
        match h {
            Ok(home) => {
                let cps_home = home;
                if !Path::new(&cps_home).is_dir() {
                    return Err(CanPiAppError::NotFound(
                        "EV CPS_HOME not a directory".to_string(),
                    ));
                }
                let mut cfg = CanpiConfig {
                    autohs_ini_path: None,
                    canpi_ini_path: None,
                    config_path: None,
                    host_port: None,
                    static_path: None,
                    template_path: None,
                };
                if let Ok(autohs) = std::env::var("AUTOHS_INI_FILE") {
                    if Path::new(&autohs).is_file() {
                        cfg.autohs_ini_path = Some(autohs);
                    } else {
                        return Err(CanPiAppError::NotFound(
                            "EV AUTOHS_INI_FILE not valid".to_string(),
                        ));
                    }
                }
                if let Ok(canpi) = std::env::var("CANPI_INI_FILE") {
                    if Path::new(&canpi).is_file() {
                        cfg.canpi_ini_path = Some(canpi);
                    } else {
                        return Err(CanPiAppError::NotFound(
                            "EV CANPI_INI_FILE not valid".to_string(),
                        ));
                    }
                }
                let cfile = cps_home.clone() + "/" + CANPI_SSR_DIR + STATIC + CFGFILE;
                if Path::new(&cfile).is_file() {
                    cfg.config_path = Some(cfile);
                }

                if let Ok(port) = std::env::var("HOST_PORT") {
                    cfg.host_port = Some(port);
                } else {
                    return Err(CanPiAppError::NotFound(
                        "EV HOST_PORT not valid".to_string(),
                    ));
                }
                let sdir = cps_home.clone() + "/" + CANPI_SSR_DIR + STATIC;
                if Path::new(&sdir).is_dir() {
                    cfg.static_path = Some(sdir);
                }
                let tdir = cps_home.clone() + "/" + CANPI_SSR_DIR + TEMPLATE;
                let grandparent = Path::new(&tdir).parent().unwrap().parent().unwrap();
                if grandparent.is_dir() {
                    cfg.template_path = Some(tdir);
                }
                return Ok(cfg);
            }
            _ => {}
        }
        Err(CanPiAppError::NotFound(
            "EV CPS_HOME not defined".to_string(),
        ))
    }
}
