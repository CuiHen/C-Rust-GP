use std::rc::Rc;
use crate::components::cgp_components::cgp_node_mutation_operators::NodeMutationOperator;
use crate::components::cgp_components::chromosome_mutation_operators::ChromosomeMutation;
use crate::components::evo_operators_for_population::mutation_operators::mutation_trait::PopulationGeneralMutation;
use crate::utils::runner::ProgramState;

pub struct PopulationMutationGeneral;

impl<T: Clone> PopulationGeneralMutation<T> for PopulationMutationGeneral {
    fn new() -> Box<dyn PopulationGeneralMutation<T>>
    where
        Self: Sized,
    {
        Box::new(Self)
    }

    fn execute(&mut self, runner: &mut ProgramState<T>,
               node_mutation_op: Rc<Box<dyn NodeMutationOperator>>,
               chromosome_mutation_op: Rc<Box<dyn ChromosomeMutation>>,
    ) {
        for id in &runner.child_ids {
            chromosome_mutation_op.execute(&mut runner.population[*id],
                                           Rc::clone(&node_mutation_op),
            );
        }
    }
}