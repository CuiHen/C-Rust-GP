use std::collections::HashSet;
use std::rc::Rc;
use petgraph::algo::toposort;
use petgraph::graph::NodeIndex;
use petgraph::prelude::StableGraph;
use nohash_hasher::BuildNoHashHasher;

use crate::components::cgp_components::cgp_node::CGPNode;
use crate::components::cgp_components::chromosome::Chromosome;
use crate::components::cgp_components::cgp_node_types::NodeType;
use crate::function_set::function_trait::Function;
use crate::utils::utility_funcs;


pub trait ChromosomeActiveNode<T> {
    fn new() -> Box<dyn ChromosomeActiveNode<T>> where Self: Sized;
    fn execute(&self, chromosome: &mut Chromosome, function_set: Rc<Vec<Box<dyn Function<T>>>>);
}


pub struct ChromosomeFindActiveNodesGeneral;

pub struct ChromosomeFindActiveNodesDAG;


impl<T> ChromosomeActiveNode<T> for ChromosomeFindActiveNodesGeneral {
    fn new() -> Box<dyn ChromosomeActiveNode<T>> where Self: Sized {
        Box::new(Self)
    }

    fn execute(&self, chromosome: &mut Chromosome, function_set: Rc<Vec<Box<dyn Function<T>>>>) {
        let params = &chromosome.params;

        let mut active: HashSet<usize, BuildNoHashHasher<usize>> = HashSet::with_capacity_and_hasher(
            params.nbr_inputs + params.graph_width + params.nbr_outputs,
            BuildNoHashHasher::default(),
        );

        let mut to_visit: Vec<usize> = vec![];
        to_visit.reserve(params.nbr_inputs + params.graph_width + params.nbr_outputs);

        // Iterate through all output node ID's
        for output_node_id in params.nbr_inputs + params.graph_width
            ..
            params.nbr_inputs + params.graph_width + params.nbr_outputs
        {
            active.insert(output_node_id);
            to_visit.push(output_node_id);
        }

        while let Some(current_node_id) = to_visit.pop() {
            let current_node: &CGPNode = &chromosome.nodes_grid[current_node_id];

            match current_node.node_type {
                NodeType::InputNode => continue,

                NodeType::ComputationalNode => {
                    let connection0 = current_node.connection0;
                    if !active.contains(&connection0) {
                        to_visit.push(connection0);
                        active.insert(connection0);
                    }

                    let inputs_needed = function_set[current_node.function_id].get_number_inputs_needed();
                    if inputs_needed == 2 {
                        let connection1 = current_node.connection1;
                        if !active.contains(&connection1) {
                            to_visit.push(connection1);
                            active.insert(connection1);
                        }
                    }
                }

                NodeType::OutputNode => {
                    let connection0 = current_node.connection0;
                    if !active.contains(&connection0) {
                        to_visit.push(connection0);
                        active.insert(connection0);
                    }
                }
            }
        }
        let mut active: Vec<usize> = active.into_iter().collect();
        active.sort_unstable();

        chromosome.active_nodes = active;
    }
}

impl<T> ChromosomeActiveNode<T> for ChromosomeFindActiveNodesDAG {
    fn new() -> Box<dyn ChromosomeActiveNode<T>> where Self: Sized {
        Box::new(Self)
    }
    fn execute(&self, chromosome: &mut Chromosome, function_set: Rc<Vec<Box<dyn Function<T>>>>) {
        let total_node_count = chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs;

        // save active nodes here
        let mut active: HashSet<usize, BuildNoHashHasher<usize>> = HashSet::default();
        active.reserve(total_node_count);

        // helper vec; which nodes must be currently visited?
        let mut to_visit: Vec<usize> = vec![];
        to_visit.reserve(total_node_count);

        // create a graph to be able to do a topological sort later
        // topo_sort is necessary, because connections can be forward and backwards
        let mut graph = StableGraph::<usize, ()>::new();

        // Fill graph with nodes
        let mut nodes: Vec<NodeIndex> = vec![];
        nodes.reserve(total_node_count);
        for i in 0..total_node_count {
            nodes.push(graph.add_node(i));
        }

        for output_node_id in (chromosome.params.nbr_inputs + chromosome.params.graph_width)..total_node_count {
            active.insert(output_node_id);
            to_visit.push(output_node_id);
        }

        while let Some(current_node_id) = to_visit.pop() {
            let current_node: &CGPNode = &chromosome.nodes_grid[current_node_id];

            match current_node.node_type {
                NodeType::InputNode => continue,

                NodeType::ComputationalNode => {
                    let connection0 = current_node.connection0;
                    graph.add_edge(nodes[connection0], nodes[current_node.position], ());

                    if !active.contains(&connection0) {
                        to_visit.push(connection0);
                        active.insert(connection0);
                    }

                    let inputs_needed = function_set[current_node.function_id].get_number_inputs_needed();
                    if inputs_needed == 2 {
                        let connection1 = current_node.connection1;
                        graph.add_edge(nodes[connection1], nodes[current_node.position], ());

                        if !active.contains(&connection1) {
                            to_visit.push(connection1);
                            active.insert(connection1);
                        }
                    }
                }

                NodeType::OutputNode => {
                    let connection0 = current_node.connection0;
                    graph.add_edge(nodes[connection0], nodes[current_node.position], ());

                    if !active.contains(&connection0) {
                        to_visit.push(connection0);
                        active.insert(connection0);
                    }
                }
            }
        }

        let inactive_nodes = (0..(chromosome.params.nbr_inputs + chromosome.params.graph_width)).collect();
        let inactive_nodes = utility_funcs::vect_difference(&inactive_nodes, &active.into_iter().collect());

        for i in inactive_nodes {
            graph.remove_node(nodes[i]);
        }

        let res = toposort(&graph, None).unwrap();
        let res = res
            .into_iter()
            .map(|node| node.index())
            .collect::<Vec<usize>>();

        chromosome.active_nodes = res;
    }
}
