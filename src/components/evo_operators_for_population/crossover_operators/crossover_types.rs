use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum CrossoverType {
    SinglePointCrossover,
    MultiPointCrossover,
    UniformCrossover,
    NoCrossover,
}

impl Display for CrossoverType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CrossoverType::SinglePointCrossover => write!(f, "One Point Crossover"),
            CrossoverType::MultiPointCrossover => write!(f, "Multi Point Crossover"),
            CrossoverType::UniformCrossover => write!(f, "Uniform Crossover"),
            CrossoverType::NoCrossover => write!(f, "No Crossover"),
        }
    }
}