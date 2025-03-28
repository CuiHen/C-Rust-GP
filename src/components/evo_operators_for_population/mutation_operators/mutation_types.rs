use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum MutationTypes {
    Point,
    Single,
    Split,
    Multi,
}

impl Display for MutationTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MutationTypes::Point => write!(f, "Point Mutation"),
            MutationTypes::Single => write!(f, "Single Mutation"),
            MutationTypes::Split => write!(f, "Split Mutation"),
            MutationTypes::Multi => write!(f, "Multi-n Mutation"),
        }
    }
}