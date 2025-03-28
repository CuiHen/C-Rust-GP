use std::rc::Rc;
use crate::components::cgp_components::cgp_node_mutation_operators::NodeMutationOperator;
use crate::components::cgp_components::chromosome_mutation_operators::ChromosomeMutation;
use crate::utils::runner::ProgramState;

pub trait PopulationGeneralMutation<T> where T: Clone
{
    fn new() -> Box<dyn PopulationGeneralMutation<T>> where Self: Sized;

    fn execute(&mut self, runner: &mut ProgramState<T>,
               node_mutation_op: Rc<Box<dyn NodeMutationOperator>>,
               chromosome_mutation_op: Rc<Box<dyn ChromosomeMutation>>,
    );
}