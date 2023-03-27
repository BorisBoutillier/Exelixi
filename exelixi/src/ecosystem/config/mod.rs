use serde::{Deserialize, Serialize};

mod ecosystem;
mod environment;
mod organism;
mod organs;
mod reproduction;
mod visualization;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConfigValue<T> {
    Fixed(T),
    Gene { min: T, max: T },
    Neuron { min: T, max: T },
}

pub use ecosystem::*;
pub use environment::*;
pub use organism::*;
pub use organs::*;
pub use reproduction::*;
pub use visualization::*;
