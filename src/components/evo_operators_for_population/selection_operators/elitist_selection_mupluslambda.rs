use crate::components::evo_operators_for_population::selection_operators::selection_trait::PopulationGeneralSelection;
use crate::utils::runner::ProgramState;
use crate::utils::utility_funcs::{get_argmins_of_value, vect_difference};

pub struct PopulationElitistSelectionMuPlusLambda;

/// Includes neutral search: Always prefer children with better or equal fitness than the parents'
impl<T: Clone> PopulationGeneralSelection<T> for PopulationElitistSelectionMuPlusLambda {
    fn new() -> Box<dyn PopulationGeneralSelection<T>> where Self: Sized {
        Box::new(Self)
    }

    fn execute(&self, runner: &mut ProgramState<T>) {
        // Get mu - many best fitness vals
        let mut sorted_fitness_vals: Vec<f32> = runner.fitness_vals_sorted.clone();
        // remove duplicates
        sorted_fitness_vals.dedup();

        let mut new_parent_ids: Vec<usize> = Vec::with_capacity(runner.params.elitists);
        for current_best_fitness_val in sorted_fitness_vals {
            let mut parent_candidate_ids = get_argmins_of_value(&runner.fitness_vals,
                                                        current_best_fitness_val);

            let remaining_new_parent_spaces = runner.params.elitists - new_parent_ids.len();
            if parent_candidate_ids.len() <= remaining_new_parent_spaces {
                // if enough space left, extend all parent candidates
                new_parent_ids.extend(parent_candidate_ids);
            } else {
                //     case: more candidates than parent/elitist spaces left
                //     remove parents from the previous generation until either all parents removed
                //     or parent_candidates.len can fill remaining spaces

                // remove parent ids until either no parent ids are left or the candidate list fits
                // into the remaining new parent set
                for old_parent_id in &runner.elitist_ids {
                    // if the old parent id is in candidate list
                    if parent_candidate_ids.contains(old_parent_id) {
                        // get index of parent in the candidate list
                        let index = parent_candidate_ids
                            .iter()
                            .position(|x| *x == *old_parent_id)
                            .unwrap();
                        // remove in O(1)
                        parent_candidate_ids.swap_remove(index);
                        // if enough parents are removed, break
                        if parent_candidate_ids.len() <= remaining_new_parent_spaces {
                            break;
                        }
                    }
                }

                // If there are still more candidates than free elitist spaces, remove the remaining
                // ones. If there are less candidates than free spaces, truncate does nothing
                parent_candidate_ids.truncate(runner.params.elitists - new_parent_ids.len());
                new_parent_ids.extend(parent_candidate_ids);

                if new_parent_ids.len() == runner.params.elitists {
                    break;
                }
            }
        }
        assert_eq!(runner.elitist_ids.len(), new_parent_ids.len());
        runner.elitist_ids = new_parent_ids;

        let child_ids: Vec<usize> = (0..runner.params.elitists + runner.params.population_size).collect();
        let child_ids = vect_difference(&child_ids, &runner.elitist_ids);
        runner.child_ids = child_ids;
    }
}