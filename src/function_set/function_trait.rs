pub trait Function<T> {
    fn get_number_inputs_needed(&self) -> usize;
    fn execute_function(&self, inputs: &[&Vec<T>])  -> Vec<T>;

}