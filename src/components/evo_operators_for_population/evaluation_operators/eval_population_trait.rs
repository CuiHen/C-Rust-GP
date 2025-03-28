use std::rc::Rc;
use crate::components::cgp_components::chromosome_evaluator_operators::ChromosomeEvaluation;
use crate::components::cgp_components::chromosome_find_active_node_operators::ChromosomeActiveNode;
use crate::function_set::function_trait::Function;
use crate::utils::runner::ProgramState;

pub trait PopulationGeneralForwardPass<T>
where
    T: Clone,
{
    fn new() -> Box<dyn PopulationGeneralForwardPass<T>>
    where
        Self: Sized;

    fn execute(&self, runner: &mut ProgramState<T>,
               evaluator_function: Rc<Box<dyn ChromosomeEvaluation<T>>>,
               active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>,
               function_set: Rc<Vec<Box<dyn Function<T>>>>);

    fn get_test_fitness(&self, runner: &mut ProgramState<T>,
                        evaluator_function: Rc<Box<dyn ChromosomeEvaluation<T>>>,
                        active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>,
                        function_set: Rc<Vec<Box<dyn Function<T>>>>) -> f32;
}

// pub trait PopulationGeneralTest<T> where T: Clone
// {
//     fn new() -> Box<dyn PopulationGeneralTest<T>> where Self: Sized;
//
//     fn execute(&self, runner: &mut Runner<T>,
//                evaluator_function: Rc<Box<dyn ChromosomeEvaluation<T>>>,
//                active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>,
//                function_set: Rc<Vec<Box<dyn Function<T>>>>) -> f32;
// }