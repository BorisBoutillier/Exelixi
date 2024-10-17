mod ecosystem;
mod environment;
mod organs;
mod reproduction;
mod species;
mod visualization;

pub use crate::ecosystem::*;
pub use ecosystem::*;
pub use organs::*;
pub use reproduction::*;
pub use species::*;
pub use visualization::*;

#[derive(Reflect, Serialize, Deserialize, Debug, Clone)]
pub enum ConfigValue<T> {
    Fixed(T),
    Gene { min: T, max: T },
    Neuron { min: T, max: T },
}
