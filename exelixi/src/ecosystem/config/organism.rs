use super::organs::{BodyConfig, EyeConfig, LocomotionConfig};
use super::reproduction::ReproductionConfig;
use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganismConfig {
    pub name: String,
    pub body: BodyConfig,
    // Optional eye vision organ
    pub eye: Option<EyeConfig>,
    // Optional eye vision organ
    pub leaf: Option<LeafConfig>,
    // Optional locomotion organ
    pub locomotion: Option<LocomotionConfig>,
    // Optional mouth organ
    pub mouth: Option<MouthConfig>,
    // Mean of creating new organism
    pub reproduction: ReproductionConfig,
    pub visualization: OrganismVisualizationConfig,
}
