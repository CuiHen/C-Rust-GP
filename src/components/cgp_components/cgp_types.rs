use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum CGPType {
    Standard,
    OriginalReorder,
    EReorder,
    LSDReorder,
    NegBiasReorder,
    UniformReorder,
    DAG,
}

impl Display for CGPType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CGPType::Standard => write!(f, "Standard CGP"),
            CGPType::OriginalReorder => write!(f, "Original Reorder"),
            CGPType::EReorder => write!(f, "Equidistant Reorder"),
            CGPType::DAG => write!(f, "DAG"),
            CGPType::LSDReorder => {write!(f, "Left Skewed Reorder")}
            CGPType::NegBiasReorder => {write!(f, "Negative Bias Reorder")}
            CGPType::UniformReorder => {write!(f, "Uniform Distribution Reorder")}
        }
    }
}