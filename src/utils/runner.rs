use std::rc::Rc;
use serde::{Serialize, Deserialize};
// use rand::prelude::ThreadRng;
use crate::global_params::CgpParameters;
use crate::components::cgp_components::chromosome::Chromosome;
use crate::components::cgp_components::chromosome_evaluator_operators::ChromosomeEvaluation;
use crate::components::cgp_components::chromosome_find_active_node_operators::ChromosomeActiveNode;
use crate::function_set::function_trait::Function;
use crate::utils::utility_funcs::{get_argmin, get_argmins_of_value, transpose, vect_difference};


// in case of mu+lambda: mu == elitsists_ids, lambda = child_ids
#[derive(Clone, Serialize, Deserialize)]
pub struct ProgramState<T>
{
    pub params: CgpParameters,
    pub data: Vec<Vec<T>>,
    pub label: Vec<Vec<T>>,
    pub eval_data: Option<Vec<Vec<T>>>,
    pub eval_label: Option<Vec<Vec<T>>>,
    pub population: Vec<Chromosome>,
    pub fitness_vals_sorted: Vec<f32>,  // helper vec to avoid multiple sorting
    pub fitness_vals: Vec<f32>,
    pub elitist_ids: Vec<usize>,
    pub child_ids: Vec<usize>,
    pub tournament_selected: Option<Vec<usize>>,
    // pub rng: ThreadRng,
}


impl<T> ProgramState<T>
where
    T: Clone,
{
    pub fn new(params: CgpParameters,
               data: Vec<Vec<T>>,
               label: Vec<Vec<T>>,
               mut eval_data: Option<Vec<Vec<T>>>,
               eval_label: Option<Vec<Vec<T>>>,
               function_set: Rc<Vec<Box<dyn Function<T>>>>,
               evaluator: Rc<Box<dyn ChromosomeEvaluation<T>>>,
               active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>,
    ) -> Self {
        // Data "must" be transposed for later evaluation cycles.
        // Doesn't really need to be, but otherwise the logic is more confusing later.
        let data = transpose(data);
        if eval_data.is_some() {
            eval_data = Some(transpose(eval_data.unwrap()));
        }

        let mut population: Vec<Chromosome> = Vec::with_capacity(params.elitists + params.population_size);
        let mut fitness_vals: Vec<f32> = Vec::with_capacity(params.elitists + params.population_size);

        for _ in 0..(params.elitists + params.population_size) {
            let mut chromosome = Chromosome::new(
                params.clone(),
            );
            let fitness = evaluator.evaluate(&mut chromosome,
                                             Rc::clone(&active_node_func),
                                             &data,
                                             &label,
                                             Rc::clone(&function_set));
            fitness_vals.push(fitness);
            population.push(chromosome);
        }

        // Get sorted fitness vals
        let mut fitness_vals_sorted = fitness_vals.clone();
        fitness_vals_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut elitist_ids: Vec<usize> = vec![];
        let mut child_ids: Vec<usize> = (0..(params.elitists + params.population_size)).collect();

        if params.elitists == 1 {
            // if there is only one parent / elitist, its search can be simplified
            let parent_id = get_argmin(&fitness_vals);
            elitist_ids = Vec::from([parent_id]);
            // this code is only valid during initialization: child_ids are a sorted list from
            // [0, 1, 2, ..., elitists + pop-size]. Thus, the index of the elitist ID in child-ids
            // is its actual ID.
            // swap_remove is O(1); child_ids needn't be sorted i guess
            child_ids.swap_remove(parent_id);
        } else {
            // case for more elitists. it works for one elitist, too. However, the computational
            // overhead is higher compared to the upper if-case
            // To get elitist IDS:
            // Reverse fitness_vals_sorted to pop the best fitness first
            let mut temp_fitness_vals_sorted: Vec<f32> = fitness_vals_sorted.clone();
            temp_fitness_vals_sorted.reverse();
            temp_fitness_vals_sorted.dedup();

            while elitist_ids.len() < params.elitists {
                let current_best_fitness_val = temp_fitness_vals_sorted.pop().unwrap();

                let mut elitist_candidates = get_argmins_of_value(&fitness_vals, current_best_fitness_val);
                elitist_ids.append(&mut elitist_candidates);
            }

            // throw away possible excessive indivudals
            elitist_ids.truncate(params.elitists);
            // find the new child IDs
            child_ids = vect_difference(&child_ids, &elitist_ids);
        }


        // let rng = rand::thread_rng();

        Self {
            params,
            data,
            label,
            eval_data,
            eval_label,
            population,
            fitness_vals_sorted,
            fitness_vals,
            elitist_ids,
            child_ids,
            // rng,
            tournament_selected: None,
        }
    }


    pub fn get_best_fitness(&self) -> f32 {
        self.fitness_vals_sorted[0]
    }


    pub fn sort_fitness_vals(&mut self) {
        let mut fitness_vals_sorted = self.fitness_vals.clone();
        fitness_vals_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.fitness_vals_sorted = fitness_vals_sorted;
    }
}

pub fn get_best_parent_chromosome<T>(runner: &ProgramState<T>) -> Chromosome {
    let idx = get_argmin(&runner.fitness_vals);
    

    runner.population[idx].clone()
}




