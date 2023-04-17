use std::collections::HashMap;

use super::organs::{BodyConfig, EyeConfig, LocomotionConfig};
use super::reproduction::ReproductionConfig;
use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpeciesConfig {
    pub name: String,
    #[serde(skip)]
    pub id: SpeciesId,
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
impl SpeciesConfig {
    pub fn update(&mut self, species_name_to_id: &HashMap<String, SpeciesId>) {
        if let Some(eye_config) = self.eye.as_mut() {
            eye_config.update(species_name_to_id);
        }
        if let Some(mouth_config) = self.mouth.as_mut() {
            mouth_config.update(species_name_to_id);
        }
    }
}

// Identifier for each different species in the simulation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SpeciesId(u8);

const UNDEFINED_SPECIES_ID: u8 = u8::MAX;
impl SpeciesId {
    pub fn new(id: u8) -> Self {
        assert!(
            id != UNDEFINED_SPECIES_ID,
            "Illegal id 0, reserved for unspecied species"
        );
        Self(id)
    }
}
impl Default for SpeciesId {
    fn default() -> Self {
        Self(UNDEFINED_SPECIES_ID)
    }
}
