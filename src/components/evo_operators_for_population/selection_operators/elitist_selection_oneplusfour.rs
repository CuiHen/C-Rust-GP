use rand::prelude::SliceRandom;
use crate::components::evo_operators_for_population::selection_operators::selection_trait::PopulationGeneralSelection;
use crate::utils::runner::ProgramState;
use crate::utils::utility_funcs::get_argmins_of_value;

pub struct PopulationElitistSelectionOnePlusFour;

impl<T: Clone> PopulationGeneralSelection<T> for PopulationElitistSelectionOnePlusFour {
    fn new() -> Box<dyn PopulationGeneralSelection<T>> where Self: Sized {
        Box::new(Self)
    }

    fn execute(&self, runner: &mut ProgramState<T>) {
        let mut rng = rand::thread_rng();
        // get the first element of the sorted fitness vals
        // that must be the lowest value, and the fitness of the parent
        // (as there is only one parent)
        let best_fitness = runner.fitness_vals_sorted[0];
        // helper Vec, contains all potential parent candidates
        let mut min_keys: Vec<usize> = get_argmins_of_value(&runner.fitness_vals, best_fitness);

        // check: if only one individual in the population is best, choose that one
        if min_keys.len() == 1 {
            runner.elitist_ids = min_keys;
        } else {
            // else: Check if the parent is in the selection of the best. If yes, remove it for
            // more diversity
            let parent_id = runner.elitist_ids[0];
            if min_keys.contains(&parent_id) {
                let index = min_keys.iter().position(|x| *x == parent_id).unwrap();
                min_keys.remove(index);
            }
            runner.elitist_ids = vec![*min_keys.choose(&mut rng).unwrap()];
        }

        runner.child_ids = (0..runner.params.elitists + runner.params.population_size).collect();
        // child ids needn't be sorted I guess -> swap_remove is O(1)
        // .remove() retains the sorting, but is O(n)
        runner.child_ids.swap_remove(runner.elitist_ids[0]);

        assert_eq!(runner.child_ids.len(), runner.params.population_size);
    }
}
