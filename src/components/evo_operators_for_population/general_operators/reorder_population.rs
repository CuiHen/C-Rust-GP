use std::rc::Rc;
use crate::components::cgp_components::chromosome_find_active_node_operators::ChromosomeActiveNode;
use crate::components::cgp_components::chromosome_reorder_operators::ChromosomeReorder;
use crate::function_set::function_trait::Function;
use crate::utils::runner::ProgramState;

pub trait GeneralReorderPopulationTrait<T> where T: Clone
{
    fn new() -> Box<dyn GeneralReorderPopulationTrait<T>> where Self: Sized;

    fn execute(&self,
               runner: &mut ProgramState<T>,
               chromosome_reorder_op: Rc<Box<dyn ChromosomeReorder<T>>>,
               active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>,
               function_set: Rc<Vec<Box<dyn Function<T>>>>);
}

pub struct ReorderPopulation;

impl<T: Clone> GeneralReorderPopulationTrait<T> for ReorderPopulation {
    fn new() -> Box<dyn GeneralReorderPopulationTrait<T>> where Self: Sized {
        Box::new(Self)
    }

    fn execute(&self,
               runner: &mut ProgramState<T>,
               chromosome_reorder_op: Rc<Box<dyn ChromosomeReorder<T>>>,
               active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>,
               function_set: Rc<Vec<Box<dyn Function<T>>>>) {
        for id in &runner.child_ids {
            chromosome_reorder_op.execute(&mut runner.population[*id],
                                          Rc::clone(&active_node_func),
                                          Rc::clone(&function_set));
        }
    }
}