mod organs;
mod reproduction;

pub use organs::*;
pub use reproduction::*;

use crate::ecosystem::*;
#[derive(Component)]
pub struct Organism {
    pub name: String,
}
