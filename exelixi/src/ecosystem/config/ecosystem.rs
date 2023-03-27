use std::path::PathBuf;

use crate::ecosystem::*;

use super::{environment::EnvironmentConfig, organism::OrganismConfig, *};
//
// Resources
//
#[derive(Serialize, Deserialize, Resource)]
pub struct EcosystemConfig {
    // Configuration information regarding the environment
    pub environment: EnvironmentConfig,
    // Configuration information regarding the organisms
    pub organisms: Vec<OrganismConfig>,
}
impl EcosystemConfig {
    pub fn from_path(path: Option<PathBuf>) -> Self {
        match path {
            None => {
                let config = ron::from_str(include_str!("../../../../configs/default.ron"))
                    .expect("default_config.ron is not correct");
                log::info!("EcosystemConfig loaded from configs/default.ron");
                config
            }
            Some(path) => {
                if let Ok(ron_string) = std::fs::read_to_string(path.as_path()) {
                    if let Ok(config) = ron::from_str::<EcosystemConfig>(&ron_string) {
                        log::info!("EcosystemConfig loaded from {:?}", path.as_os_str());
                        config
                    } else {
                        log::error!("EcosystemConfig could not be loaded from {:?}, invalid content in the file.",path.as_os_str());
                        panic!();
                    }
                } else {
                    log::error!(
                        "EcosystemConfig could not be loaded from {:?}, file does not exists.",
                        path.as_os_str()
                    );
                    panic!();
                }
            }
        }
    }
}
