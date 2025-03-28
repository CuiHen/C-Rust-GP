use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum NodeType {
    InputNode,
    ComputationalNode,
    OutputNode,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::InputNode => write!(f, "Input_Node"),
            NodeType::ComputationalNode => write!(f, "Computational_Node"),
            NodeType::OutputNode => write!(f, "Output_Node"),
        }
    }
}