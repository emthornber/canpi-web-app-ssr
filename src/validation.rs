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
    pub config_path: String,
    pub host_port: String,
    pub static_path: String,
    pub template_path: String,
    pub pkg_defn: Pkg,
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
    /// then an error result is returned.
    ///
    /// If the EV HOST_PORT is not defined then the entry in the struct is set to "8080".
    ///
    pub fn new() -> Result<CanpiConfig, CanPiAppError> {
        let h = std::env::var("CPSSR_HOME");
        if let Ok(home) = h {
            let cps_home = home;
            if !Path::new(&cps_home).is_dir() {
                return Err(CanPiAppError::NotFound(
                    "EV CPSSR_HOME not a directory".to_string(),
                ));
            }

            let cfile = cps_home.clone() + "/" + STATIC + CFGFILE;
            if !Path::new(&cfile).is_file() {
                return Err(CanPiAppError::NotFound(format!(
                    "Configuration file '{cfile}' not found"
                )));
            }

            let pkg = Pkg::new(&cfile);

            let port = std::env::var("HOST_PORT").unwrap_or_else(|e| {
                log::warn!("HOST_PORT not defined, using default 8080: {}", e);
                "8080".to_string()
            });

            let sdir = cps_home.clone() + "/" + STATIC;
            if !Path::new(&sdir).is_dir() {
                return Err(CanPiAppError::NotFound(format!(
                    "Configuration directory '{sdir}' not found",
                )));
            }

            let tdir = cps_home.clone() + "/" + TEMPLATE;
            let grandparent = Path::new(&tdir).parent().unwrap().parent().unwrap();
            if !grandparent.is_dir() {
                return Err(CanPiAppError::NotFound(format!(
                    "Configuration directory '{tdir}' not found",
                )));
            }

            let cfg = CanpiConfig {
                config_path: cfile,
                host_port: port,
                static_path: sdir,
                template_path: tdir,
                pkg_defn: pkg,
            };
            Ok(cfg)
        } else {
            Err(CanPiAppError::NotFound(
                "EV CPSSR_HOME not defined".to_string(),
            ))
        }
    }
}
