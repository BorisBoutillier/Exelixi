use crate::ecosystem::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OrganismKind {
    Plant,
    Herbivore,
}
#[derive(Component)]
pub struct Organism {
    pub kind: OrganismKind,
}
