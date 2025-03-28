use std::rc::Rc;
use crate::components::cgp_components::chromosome_find_active_node_operators::ChromosomeActiveNode;
use crate::components::evo_operators_for_population::crossover_operators::crossover_algos::Crossover;
use crate::function_set::function_trait::Function;
use crate::utils::runner::ProgramState;

pub trait PopulationGeneralCrossover<T> where T: Clone
{
    fn new() -> Box<dyn PopulationGeneralCrossover<T>> where Self: Sized;

    fn execute(&self,
               runner: &mut ProgramState<T>,
               active_node_function: Rc<Box<dyn ChromosomeActiveNode<T>>>,
               function_set: Rc<Vec<Box<dyn Function<T>>>>,
               crossover_operator: Rc<Box<dyn Crossover<T>>>
    );
}