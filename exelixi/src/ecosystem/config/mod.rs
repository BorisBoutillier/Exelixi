use serde::{Deserialize, Serialize};

mod ecosystem;
mod environment;
mod organs;
mod reproduction;
mod species;
mod visualization;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConfigValue<T> {
    Fixed(T),
    Gene { min: T, max: T },
    Neuron { min: T, max: T },
}

pub use ecosystem::*;
pub use environment::*;
pub use organs::*;
pub use reproduction::*;
pub use species::*;
pub use visualization::*;
