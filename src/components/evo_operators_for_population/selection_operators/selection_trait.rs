use crate::utils::runner::ProgramState;

pub trait PopulationGeneralSelection<T>
{
    fn new() -> Box<dyn PopulationGeneralSelection<T>> where Self: Sized;

    fn execute(&self, runner: &mut ProgramState<T>);
}