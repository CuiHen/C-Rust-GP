use rand::distributions::Uniform;
use rand::{Rng, thread_rng};
use rand::distributions::Distribution;
use crate::components::cgp_components::cgp_node::CGPNode;
use crate::components::cgp_components::cgp_node_types::NodeType;
use crate::utils::cycle_checker::CGPEdges;
use crate::utils::utility_funcs::gen_random_number_for_node;

pub trait NodeMutationOperator {
    fn new() -> Box<dyn NodeMutationOperator> where Self: Sized;

    #[allow(unused_variables)]
    fn mutate_standard(&self, node: &mut CGPNode) { unimplemented!("Placeholder function; Mutation is configured wrong somewhere!") }

    #[allow(unused_variables)]
    fn mutate_dag(&self, node: &mut CGPNode, cgp_edges: &mut CGPEdges) {unimplemented!("Placeholder function; Mutation is configured wrong somewhere!")}
}

pub struct NodeMutationGeneral;

pub struct NodeMutationDAG;


impl NodeMutationOperator for NodeMutationGeneral {
    fn new() -> Box<dyn NodeMutationOperator> where Self: Sized {
        Box::new(Self)
    }

    fn mutate_standard(&self, node: &mut CGPNode) {
        assert!(node.node_type != NodeType::InputNode);

        match node.node_type {
            NodeType::OutputNode => self.mutate_output_node(node),
            NodeType::ComputationalNode => self.mutate_computational_node(node),
            _ => { panic!("Trying to mutate input node") }
        }
    }
}

impl NodeMutationGeneral {
    fn mutate_output_node(&self, node: &mut CGPNode) {
        node.connection0 = gen_random_number_for_node(node.connection0,
                                                      node.graph_width + node.nbr_inputs);

        assert!(node.connection0 < node.position);
    }
    fn mutate_computational_node(&self, node: &mut CGPNode) {
        let rand_nbr = thread_rng().gen_range(0..=2);
        match rand_nbr {
            0 => {
                node.connection0 = gen_random_number_for_node(node.connection0,
                                                              node.position);
            }

            1 => {
                node.connection1 = gen_random_number_for_node(node.connection1,
                                                              node.position);
            }

            2 => self.mutate_function(node),

            _ => { panic!("Mutation of comp node somehow broken?") }
        };

        assert!(node.connection0 < node.position);
        assert!(node.connection1 < node.position);
    }
    fn mutate_function(&self, node: &mut CGPNode) {
        node.function_id = gen_random_number_for_node(node.function_id, node.number_functions);
    }
}

impl NodeMutationOperator for NodeMutationDAG {
    fn new() -> Box<dyn NodeMutationOperator> where Self: Sized {
        Box::new(Self)
    }

    fn mutate_dag(&self, node: &mut CGPNode, cgp_edges: &mut CGPEdges) {
        match node.node_type {
            NodeType::OutputNode => self.mutate_output_node(node),
            NodeType::ComputationalNode => self.mutate_computational_node(node, cgp_edges),
            _ => { panic!("Trying to mutate input node") }
        }
    }
}

impl NodeMutationDAG {
    fn mutate_output_node(&self, node: &mut CGPNode) {
        loop {
            let rand_nbr: usize = thread_rng().gen_range(0..(node.nbr_inputs + node.graph_width));

            if rand_nbr != node.connection0 {
                node.connection0 = rand_nbr;
                break;
            }
        }
    }

    fn mutate_computational_node(&self, node: &mut CGPNode, cgp_edges: &mut CGPEdges) {
        let rand_nbr = rand::thread_rng().gen_range(0..=2);
        match rand_nbr {
            0 => {
                let new_connection_id = self.gen_random_connection_id(node.connection0,
                                                                      node.position,
                                                                      node.nbr_inputs + node.graph_width,
                                                                      cgp_edges);

                cgp_edges.remove_edge(node.position, node.connection0);
                cgp_edges.add_edge(node.position, new_connection_id);

                node.connection0 = new_connection_id;
            }

            1 => {
                let new_connection_id = self.gen_random_connection_id(node.connection1,
                                                                      node.position,
                                                                      node.nbr_inputs + node.graph_width,
                                                                      cgp_edges);

                cgp_edges.remove_edge(node.position, node.connection1);
                cgp_edges.add_edge(node.position, new_connection_id);

                node.connection1 = new_connection_id;
            }

            2 => node.function_id = self.gen_random_function_id(node.function_id,
                                                                node.number_functions),

            _ => { panic!("Mutation: output node something wrong") }
        };
    }


    fn gen_random_connection_id(&self,
                                previous_connection: usize,
                                position: usize,
                                upper_range: usize,
                                cgp_edges: &CGPEdges) -> usize {
        let between = Uniform::from(0..upper_range);
        let mut rng = thread_rng();

        loop {
            let rand_nbr: usize = between.sample(&mut rng);

            if (rand_nbr != previous_connection) && (rand_nbr != position) {
                // if rand_nbr != position {
                if !cgp_edges.leads_to_cycle(position, rand_nbr) {
                    return rand_nbr;
                }
            }
        }
    }

    fn gen_random_function_id(&self, excluded: usize, upper_range: usize) -> usize {
        loop {
            let mut rng = thread_rng();
            let rand_nbr: usize = rng.gen_range(0..upper_range);

            if rand_nbr != excluded {
                return rand_nbr;
            }
        }
    }
}
