//! Implementations for a Node in a CGP Graph.
//! `CGPNode` contains the parametrization of a single node.
//! `MutateNodeStandard` and/or `MutateNodeDAG` contain the mutational logic of a node.
//! I.e. How does mutation affect the node.

use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::components::cgp_components::cgp_node_types::NodeType;
use crate::utils::cycle_checker::CGPEdges;


#[derive(Clone, Serialize, Deserialize)]
pub struct CGPNode {
    pub position: usize,
    pub node_type: NodeType,
    pub nbr_inputs: usize,
    pub graph_width: usize,
    pub function_id: usize,
    pub connection0: usize,
    pub connection1: usize,
    pub number_functions: usize,
}


impl CGPNode {
    pub fn new(position: usize,
               nbr_inputs: usize,
               graph_width: usize,
               node_type: NodeType,
               number_functions: usize,
               cgp_edges: &mut Option<CGPEdges>) -> Self {
        let function_id: usize;
        let connection0: usize;
        let connection1: usize;

        match node_type {
            NodeType::InputNode => {
                function_id = usize::MAX;
                connection0 = usize::MAX;
                connection1 = usize::MAX;
            }
            NodeType::ComputationalNode => {
                function_id = rand::thread_rng().gen_range(0..number_functions);
                connection0 = rand::thread_rng().gen_range(0..position);
                connection1 = rand::thread_rng().gen_range(0..position);

                if cgp_edges.is_some() {
                    cgp_edges.as_mut().unwrap().add_edge(position, connection0);
                    cgp_edges.as_mut().unwrap().add_edge(position, connection1);
                }
            }
            NodeType::OutputNode => {
                function_id = usize::MAX;
                connection0 = rand::thread_rng().gen_range(0..nbr_inputs + graph_width);
                connection1 = usize::MAX;
            }
        }

        Self {
            position,
            node_type,
            nbr_inputs,
            graph_width,
            function_id,
            connection0,
            connection1,
            number_functions,
        }
    }
}

