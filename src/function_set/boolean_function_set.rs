use std::rc::Rc;
use crate::function_set::function_trait::Function;


pub fn get_boolean_function_set() -> Rc<Vec<Box<dyn Function<bool>>>> {
    let mut function_set: Vec<Box<dyn Function<bool>>> = vec![];

    function_set.push(Box::new(BoolAND));
    function_set.push(Box::new(BoolOR));
    function_set.push(Box::new(BoolNAND));
    function_set.push(Box::new(BoolNOR));

    Rc::new(function_set)
}


pub struct BoolAND;

pub struct BoolOR;

pub struct BoolNAND;

pub struct BoolNOR;

impl Function<bool> for BoolAND {
    fn get_number_inputs_needed(&self) -> usize {
        2
    }

    fn execute_function(&self, inputs: &[&Vec<bool>]) -> Vec<bool> {
        let input0 = inputs[0];
        let input1 = inputs[1];
        return input0
            .iter()
            .zip(input1.iter())
            .map(|(a, b)| *a & *b)
            .collect();
    }
}

impl Function<bool> for BoolOR {
    fn get_number_inputs_needed(&self) -> usize {
        2
    }

    fn execute_function(&self, inputs: &[&Vec<bool>]) -> Vec<bool> {
        let input0 = inputs[0];
        let input1 = inputs[1];
        return input0
            .iter()
            .zip(input1.iter())
            .map(|(a, b)| *a | *b)
            .collect();
    }
}

impl Function<bool> for BoolNAND {

    fn get_number_inputs_needed(&self) -> usize {
        2
    }

    fn execute_function(&self, inputs: &[&Vec<bool>]) -> Vec<bool> {
        let input0 = inputs[0];
        let input1 = inputs[1];
        return input0
            .iter()
            .zip(input1.iter())
            .map(|(a, b)| !(*a & *b))
            .collect();
    }
}

impl Function<bool> for BoolNOR {

    fn get_number_inputs_needed(&self) -> usize {
        2
    }

    fn execute_function(&self, inputs: &[&Vec<bool>]) -> Vec<bool> {
        let input0 = inputs[0];
        let input1 = inputs[1];
        return input0
            .iter()
            .zip(input1.iter())
            .map(|(a, b)| !(*a | *b))
            .collect();
    }
}


