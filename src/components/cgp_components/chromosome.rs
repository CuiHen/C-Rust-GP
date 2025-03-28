//! The chromosome class; a chromosome defined by a CGP graph.
//! It contains nodes in a single grid-line.

use serde::{Deserialize, Serialize};
use crate::global_params::CgpParameters;
use crate::components::cgp_components::cgp_node::CGPNode;
use crate::components::cgp_components::cgp_types::CGPType;
use crate::components::cgp_components::cgp_node_types::NodeType;
use crate::utils::cycle_checker::CGPEdges;


#[derive(Clone, Serialize, Deserialize)]
pub struct Chromosome
{
    pub params: CgpParameters,
    pub nodes_grid: Vec<CGPNode>,
    pub active_nodes: Vec<usize>,
    pub cgp_edges: Option<CGPEdges>, // only used for DAG
}



impl Chromosome {
    pub fn new(params: CgpParameters) -> Self {
        let mut nodes_grid: Vec<CGPNode> = vec![];
        nodes_grid.reserve(params.nbr_inputs + params.graph_width + params.nbr_outputs);

        let mut cgp_edges: Option<CGPEdges>;
        if params.cgp_type == CGPType::DAG {
            cgp_edges = Some(
                    CGPEdges::new(params.nbr_inputs + params.graph_width)
            );
        } else {
            cgp_edges = None;
        }

        // input nodes
        for position in 0..params.nbr_inputs {
            nodes_grid.push(CGPNode::new(position,
                                         params.nbr_inputs,
                                         params.graph_width,
                                         NodeType::InputNode,
                                         params.number_functions,
                                         &mut cgp_edges,
            ));
        }
        // computational nodes
        for position in params.nbr_inputs..(params.nbr_inputs + params.graph_width) {
            nodes_grid.push(CGPNode::new(position,
                                         params.nbr_inputs,
                                         params.graph_width,
                                         NodeType::ComputationalNode,
                                         params.number_functions,
                                         &mut cgp_edges,
            ));
        }
        // output nodes
        for position in (params.nbr_inputs + params.graph_width)
            ..
            (params.nbr_inputs + params.graph_width + params.nbr_outputs) {
            nodes_grid.push(CGPNode::new(position,
                                         params.nbr_inputs,
                                         params.graph_width,
                                         NodeType::OutputNode,
                                         params.number_functions,
                                         &mut cgp_edges,

            ));
        }

        Self {
            params,
            nodes_grid,
            active_nodes: vec![],
            cgp_edges,
        }
    }
}

