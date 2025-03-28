use rand::prelude::SliceRandom;
use crate::utils::runner::ProgramState;

pub trait ClonePopulation<T> where T: Clone
{
    fn new() -> Box<dyn ClonePopulation<T>> where Self: Sized;

    fn execute(&self, runner: &mut ProgramState<T>);
}

pub struct CloneParentToChild;

/// Must be used when no crossover is applied
/// Similar outcome can be achieved by setting `crossover_rate = 0`. But that's more efficient I think
/// And more readable
impl<T: Clone> ClonePopulation<T> for CloneParentToChild {
    fn new() -> Box<dyn ClonePopulation<T>> where Self: Sized {
        Box::new(Self)
    }


    fn execute(&self, runner: &mut ProgramState<T>) {
        let mut rng = rand::thread_rng();

        for id in &runner.child_ids {
            // get the parent Id, so it can be cloned
            let parent_id: usize;
            if runner.params.elitists == 1 {
                // case: (1+4) ES; only one parent available to clone
                parent_id = runner.elitist_ids[0];
            } else {
                parent_id = *runner.elitist_ids.choose(&mut rng).unwrap();
            }
            runner.population[*id] = runner.population[parent_id].clone();
        }
    }
}