//! Handles the evaluation of a chromosome given inputs and respective outputs.

use std::collections::HashMap;
use std::rc::Rc;
use nohash_hasher::BuildNoHashHasher;
use crate::components::cgp_components::cgp_node::CGPNode;
use crate::components::cgp_components::chromosome::Chromosome;
use crate::components::cgp_components::cgp_node_types::NodeType;
use crate::utils::fitness_metrics;
use crate::utils::utility_funcs::transpose;
use crate::components::cgp_components::chromosome_find_active_node_operators::ChromosomeActiveNode;
use crate::function_set::function_trait::Function;

pub trait ChromosomeEvaluation<T> where T: Clone {
    fn new() -> Box<dyn ChromosomeEvaluation<T>> where Self: Sized;
    fn evaluate(&self,
                chromosome: &mut Chromosome,
                active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>,
                inputs: &Vec<Vec<T>>,
                labels: &Vec<Vec<T>>,
                function_set: Rc<Vec<Box<dyn Function<T>>>>,
    ) -> f32;
}


#[derive(Clone)]
pub struct ChromosomeEvaluatorGeneral;


impl ChromosomeEvaluation<f32> for ChromosomeEvaluatorGeneral {
    fn new() -> Box<dyn ChromosomeEvaluation<f32>> {
        Box::new(Self)
    }

    fn evaluate(&self,
                chromosome: &mut Chromosome,
                active_node_func: Rc<Box<dyn ChromosomeActiveNode<f32>>>,
                inputs: &Vec<Vec<f32>>,
                labels: &Vec<Vec<f32>>,
                function_set: Rc<Vec<Box<dyn Function<f32>>>>,
    ) -> f32 {
        let mut outputs = self.forward_pass(chromosome, active_node_func, inputs, Rc::clone(&function_set));

        let output_start_id = chromosome.params.nbr_inputs + chromosome.params.graph_width;
        let outs: Vec<Vec<f32>> = vec![outputs.remove(&output_start_id).unwrap()];
        

        fitness_metrics::fitness_regression(&outs, labels)
    }
}

impl ChromosomeEvaluation<bool> for ChromosomeEvaluatorGeneral {
    fn new() -> Box<dyn ChromosomeEvaluation<bool>> {
        Box::new(Self)
    }
    fn evaluate(&self,
                chromosome: &mut Chromosome,
                active_node_func: Rc<Box<dyn ChromosomeActiveNode<bool>>>,
                inputs: &Vec<Vec<bool>>,
                labels: &Vec<Vec<bool>>,
                function_set: Rc<Vec<Box<dyn Function<bool>>>>,
    ) -> f32 {
        let mut outputs = self.forward_pass(chromosome, Rc::clone(&active_node_func), inputs, Rc::clone(&function_set));

        let output_start_id = chromosome.params.nbr_inputs + chromosome.params.graph_width;
        let output_end_id = chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs;

        let mut outs: Vec<Vec<bool>> = Vec::with_capacity(output_end_id - output_start_id);
        for i in output_start_id..output_end_id {
            outs.push(outputs.remove(&i).unwrap());
        }

        let outs = transpose(outs);
        
        fitness_metrics::fitness_boolean(&outs, labels)
    }
}


impl ChromosomeEvaluatorGeneral {
    fn forward_pass<T: Clone>(
        &self,
        chromosome: &mut Chromosome,
        active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>,
        inputs: &Vec<Vec<T>>,
        function_set: Rc<Vec<Box<dyn Function<T>>>>,
    )
        -> HashMap<usize, Vec<T>, BuildNoHashHasher<usize>>
    {
        active_node_func.execute(chromosome, Rc::clone(&function_set));

        let mut outputs: HashMap<usize, Vec<T>, BuildNoHashHasher<usize>> = HashMap::with_capacity_and_hasher(
            chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs,
            BuildNoHashHasher::default(),
        );

        // iterate through each input and calculate for each new vector its output
        // as the inputs are transposed, the n-th element of the whole dataset is input
        // i.e. given a dataset with 3 datapoints per entry; and 5 entries.
        // then it will input the first datapoint of all 5 entries first. Then the second, etc.
        for node_id in &chromosome.active_nodes {
            // println!("{:?}", input_slice);
            let current_node: &CGPNode = &chromosome.nodes_grid[*node_id];

            match current_node.node_type {
                NodeType::InputNode => {
                    outputs.insert(*node_id, inputs[*node_id].clone());
                }
                NodeType::OutputNode => {
                    let con1 = current_node.connection0;
                    let prev_output1 = outputs.get(&con1).unwrap();
                    outputs.insert(*node_id, prev_output1.clone());
                }
                NodeType::ComputationalNode => {
                    let prev_output1 = outputs.get(&current_node.connection0).unwrap();
                    let calculated_result: Vec<T>;
                    let function = &function_set[current_node.function_id];

                    if function.get_number_inputs_needed() == 2 {
                        let prev_output2 = outputs.get(&current_node.connection1).unwrap();
                        calculated_result = function.execute_function(&[prev_output1, prev_output2]);
                    } else {
                        calculated_result = function.execute_function(&[prev_output1]);
                    }
                    outputs.insert(*node_id, calculated_result);
                }
            }
        }

        outputs
    }
}