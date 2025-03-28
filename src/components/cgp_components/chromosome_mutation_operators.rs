use std::rc::Rc;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use crate::components::cgp_components::cgp_node_mutation_operators::NodeMutationOperator;
use crate::components::cgp_components::cgp_types::CGPType;
use crate::components::cgp_components::chromosome::Chromosome;

pub trait ChromosomeMutation {
    fn new() -> Box<dyn ChromosomeMutation>
    where
        Self: Sized;
    fn execute(&self,
               chromosome: &mut Chromosome,
               mutate_function: Rc<Box<dyn NodeMutationOperator>>);
}

pub struct ChromosomeMutationSingle;

pub struct ChromosomeMutationPoint;

pub struct ChromosomeMutationMultiN;

pub struct ChromosomeMutationSplit;


impl ChromosomeMutation for ChromosomeMutationSingle {
    fn new() -> Box<dyn ChromosomeMutation>
    where
        Self: Sized,
    {
        Box::new(Self)
    }

    fn execute(&self, chromosome: &mut Chromosome, mutate_function: Rc<Box<dyn NodeMutationOperator>>) {
        let start_id = chromosome.params.nbr_inputs;
        let end_id = chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs;

        let between = Uniform::from(start_id..end_id);
        let mut rng = rand::thread_rng();

        let mut mutated_nodes: Vec<usize> = Vec::with_capacity(128);

        loop {
            let random_node_id = between.sample(&mut rng);

            mutated_nodes.push(random_node_id);

            if chromosome.params.cgp_type == CGPType::DAG {
                mutate_function.mutate_dag(&mut chromosome.nodes_grid[random_node_id], chromosome.cgp_edges.as_mut().unwrap());
            } else {
                mutate_function.mutate_standard(&mut chromosome.nodes_grid[random_node_id]);
            }
            if chromosome.active_nodes.contains(&random_node_id) {
                break;
            }
        }
    }
}

impl ChromosomeMutation for ChromosomeMutationPoint {
    fn new() -> Box<dyn ChromosomeMutation>
    where
        Self: Sized,
    {
        Box::new(Self)
    }

    fn execute(&self, chromosome: &mut Chromosome, mutate_function: Rc<Box<dyn NodeMutationOperator>>) {
        let start_id = chromosome.params.nbr_inputs;
        let end_id = chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs;

        let mut rng = rand::thread_rng();
        let between = Uniform::new(0., 1.);

        for node_id in start_id..end_id {
            let rand_val = between.sample(&mut rng);
            if rand_val <= chromosome.params.mutation_rate {
                if chromosome.params.cgp_type == CGPType::DAG {
                    mutate_function.mutate_dag(&mut chromosome.nodes_grid[node_id], chromosome.cgp_edges.as_mut().unwrap());
                } else {
                    mutate_function.mutate_standard(&mut chromosome.nodes_grid[node_id]);
                }
            }
        }
    }
}

impl ChromosomeMutation for ChromosomeMutationMultiN {
    fn new() -> Box<dyn ChromosomeMutation>
    where
        Self: Sized,
    {
        Box::new(Self)
    }

    fn execute(&self, chromosome: &mut Chromosome, mutate_function: Rc<Box<dyn NodeMutationOperator>>) {
        let start_id = chromosome.params.nbr_inputs;
        let end_id = chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs;

        let between = Uniform::from(start_id..end_id);
        let mut rng = rand::thread_rng();

        let mut mutated_active_nodes_counter = 0;

        let mut mutated_nodes: Vec<usize> = Vec::with_capacity(128);

        loop {
            let random_node_id = between.sample(&mut rng);
            mutated_nodes.push(random_node_id);

            if chromosome.params.cgp_type == CGPType::DAG {
                mutate_function.mutate_dag(&mut chromosome.nodes_grid[random_node_id], chromosome.cgp_edges.as_mut().unwrap());
            } else {
                mutate_function.mutate_standard(&mut chromosome.nodes_grid[random_node_id]);
            }

            if chromosome.active_nodes.contains(&random_node_id) {
                mutated_active_nodes_counter += 1;
            }

            if mutated_active_nodes_counter >= chromosome.params.multi_n_mutations {
                break;
            }
        }
    }
}

impl ChromosomeMutation for ChromosomeMutationSplit {
    fn new() -> Box<dyn ChromosomeMutation>
    where
        Self: Sized,
    {
        Box::new(Self)
    }

    fn execute(&self, chromosome: &mut Chromosome, mutate_function: Rc<Box<dyn NodeMutationOperator>>) {
        let start_id = chromosome.params.nbr_inputs;
        let end_id = chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs;

        let mut rng = rand::thread_rng();
        let between = Uniform::new(0., 1.);

        for node_id in start_id..end_id {
            let rand_val = between.sample(&mut rng);

            let mut mutate_flag = false;
            // Check if a node is active or not
            if chromosome.active_nodes.contains(&node_id) {
                if rand_val <= chromosome.params.split_mutation_rate_active {
                    mutate_flag = true;
                }
            } else if rand_val <= chromosome.params.split_mutation_rate_inactive {
                mutate_flag = true
            }

            if mutate_flag {
                if chromosome.params.cgp_type == CGPType::DAG {
                    mutate_function.mutate_dag(&mut chromosome.nodes_grid[node_id], chromosome.cgp_edges.as_mut().unwrap());
                } else {
                    mutate_function.mutate_standard(&mut chromosome.nodes_grid[node_id]);
                }
            }
        }
    }
}