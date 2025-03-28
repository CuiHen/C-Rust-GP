use std::rc::Rc;
use crate::components::cgp_components::chromosome::Chromosome;
use crate::components::cgp_components::chromosome_find_active_node_operators::ChromosomeActiveNode;
use crate::components::evo_operators_for_population::crossover_operators::crossover_algos::*;
use crate::components::evo_operators_for_population::crossover_operators::crossover_trait::PopulationGeneralCrossover;
use crate::function_set::function_trait::Function;
use crate::utils::runner::ProgramState;
use crate::utils::utility_funcs::vect_difference;

pub struct PopulationCrossoverTournament;


impl<T: Clone> PopulationGeneralCrossover<T> for PopulationCrossoverTournament {
    fn new() -> Box<dyn PopulationGeneralCrossover<T>> where Self: Sized {
        Box::new(Self)
    }

    fn execute(&self, runner: &mut ProgramState<T>,
               active_node_function: Rc<Box<dyn ChromosomeActiveNode<T>>>,
               function_set: Rc<Vec<Box<dyn Function<T>>>>,
               crossover_operator: Rc<Box<dyn Crossover<T>>>,
    ) {
        // get all new children ids; i.e. the ID's of chromosomes in the population that
        // can be replaced.
        // It must exclude the elitists, otherwise they may be replaced too
        let children_set: Vec<usize> = (0..(runner.params.population_size + runner.params.elitists)).collect();
        let children_set: Vec<usize> = vect_difference(&children_set, &runner.elitist_ids);

        // create new population
        let mut new_population: Vec<Chromosome> = runner.population.clone();

        for (i, child_ids) in children_set.chunks(2).enumerate() {
            let crossover_prob = rand::random::<f32>();
            let parent1_id = runner.tournament_selected.as_deref().unwrap()[2 * i];
            let parent2_id = runner.tournament_selected.as_deref().unwrap()[2 * i + 1];

            if crossover_prob <= runner.params.crossover_rate {
                crossover_operator.execute(runner,
                                           &mut new_population,
                                           Rc::clone(&active_node_function),
                                           Rc::clone(&function_set),
                                           child_ids[0],
                                           child_ids[1],
                                           parent1_id,
                                           parent2_id)
                // match runner.params.crossover_type {
                //     CrossoverType::SinglePointCrossover => single_point_crossover(runner,
                //                                                                   &mut new_population,
                //                                                                   Rc::clone(&active_node_function),
                //                                                                   Rc::clone(&function_set),
                //                                                                   child_ids[0],
                //                                                                   child_ids[1],
                //                                                                   parent1_id,
                //                                                                   parent2_id),
                //     CrossoverType::MultiPointCrossover => multi_point_crossover(runner,
                //                                                                 &mut new_population,
                //                                                                 Rc::clone(&active_node_function),
                //                                                                 Rc::clone(&function_set),
                //                                                                 child_ids[0],
                //                                                                 child_ids[1],
                //                                                                 parent1_id,
                //                                                                 parent2_id),
                //     CrossoverType::UniformCrossover => uniform_crossover(runner,
                //                                                          &mut new_population,
                //                                                          Rc::clone(&active_node_function),
                //                                                          Rc::clone(&function_set),
                //                                                          child_ids[0],
                //                                                          child_ids[1],
                //                                                          parent1_id,
                //                                                          parent2_id),
                //     CrossoverType::NoCrossover => no_crossover(runner,
                //                                                &mut new_population,
                //                                                child_ids[0],
                //                                                child_ids[1],
                //                                                parent1_id,
                //                                                parent2_id),
                // }
            } else {
                //     no crossover, just copy parents
                new_population[child_ids[0]] = runner.population[parent1_id].clone();
                new_population[child_ids[1]] = runner.population[parent2_id].clone();
            }
        }
        runner.population = new_population;
    }
}

