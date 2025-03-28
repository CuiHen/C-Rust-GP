use rand::prelude::IteratorRandom;
use crate::components::evo_operators_for_population::selection_operators::selection_trait::PopulationGeneralSelection;
use crate::utils::runner::ProgramState;
use crate::utils::utility_funcs::{get_argmins_of_value, vect_difference};

pub struct PopulationElitistSelectionWithTournament;


/// Pure elitist selecion; without neutral search.
/// Used in the case of tournament selection to save the elitists
impl<T: Clone> PopulationGeneralSelection<T> for PopulationElitistSelectionWithTournament {
    fn new() -> Box<dyn PopulationGeneralSelection<T>> where Self: Sized {
        Box::new(Self)
    }

    fn execute(&self, runner: &mut ProgramState<T>) {
        assert!(runner.params.tournament_size > 0);
        let mut rng = rand::thread_rng();
        // Elitists:
        let mut temp_fitness_vals_sorted = runner.fitness_vals_sorted.clone();
        // reverse to pop the last element - the best one
        temp_fitness_vals_sorted.dedup();
        temp_fitness_vals_sorted.reverse();

        let mut elitist_ids: Vec<usize> = vec![];

        while elitist_ids.len() < runner.params.elitists {
            let current_best_fitness_val = temp_fitness_vals_sorted.pop().unwrap();

            let mut elitist_candidates = get_argmins_of_value(&runner.fitness_vals,
                                                              current_best_fitness_val);
            elitist_ids.append(&mut elitist_candidates)
        }

        elitist_ids.truncate(runner.params.elitists);
        runner.elitist_ids = elitist_ids;

        let child_ids: Vec<usize> = (0..runner.params.elitists + runner.params.population_size).collect();
        let child_ids = vect_difference(&child_ids, &runner.elitist_ids);
        runner.child_ids = child_ids;

    //     Tournament:
        let mut selection = vec![];

        for _ in 0..runner.params.population_size {
            let winner_id = runner.fitness_vals
                .clone()
                .into_iter()
                .enumerate() // get tuples: (i, fitness_val) with i := chromosome id
                .choose_multiple(&mut rng, runner.params.tournament_size)
                .into_iter()
                .min_by(|i, j| i.1.partial_cmp(&j.1).unwrap())  // Sort by fitness val
                .map(|(i, _)| i)// get id of chromosome
                .unwrap();

            selection.push(winner_id)
        }
        runner.tournament_selected = Some(selection);
    }
}

