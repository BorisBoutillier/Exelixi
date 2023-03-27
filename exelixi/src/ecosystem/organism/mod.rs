mod kind;
mod organs;
mod reproduction;

pub use kind::*;
pub use organs::*;
pub use reproduction::*;

use crate::ecosystem::*;
#[derive(Component)]
pub struct Organism {
    pub kind: OrganismKind,
    pub name: String,
}
