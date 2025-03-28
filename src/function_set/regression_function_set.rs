use std::rc::Rc;
use crate::function_set::function_trait::Function;
use float_eq::float_eq;

pub fn get_regression_function_set() -> Rc<Vec<Box<dyn Function<f32>>>> {
    let mut function_set: Vec<Box<dyn Function<f32>>> = vec![];

    function_set.push(Box::new(RegressionAdd));
    function_set.push(Box::new(RegressionSub));
    function_set.push(Box::new(RegressionMul));
    function_set.push(Box::new(RegressionDiv));
    function_set.push(Box::new(RegressionSin));
    function_set.push(Box::new(RegressionCos));
    function_set.push(Box::new(RegressionLn));
    function_set.push(Box::new(RegressionExp));

    Rc::new(function_set)
}

pub struct RegressionAdd;

pub struct RegressionSub;

pub struct RegressionMul;

pub struct RegressionDiv;

pub struct RegressionSin;

pub struct RegressionCos;

pub struct RegressionLn;

pub struct RegressionExp;


impl Function<f32> for RegressionAdd {

    fn get_number_inputs_needed(&self) -> usize {
        2
    }

    fn execute_function(&self, inputs: &[&Vec<f32>]) -> Vec<f32> {
        let input0 = inputs[0];
        let input1 = inputs[1];
        return input0
            .iter()
            .zip(input1.iter())
            .map(|(a, b)| *a + *b)
            .collect();
    }
}

impl Function<f32> for RegressionSub {

    fn get_number_inputs_needed(&self) -> usize {
        2
    }

    fn execute_function(&self, inputs: &[&Vec<f32>]) -> Vec<f32> {
        let input0 = inputs[0];
        let input1 = inputs[1];
        return input0
            .iter()
            .zip(input1.iter())
            .map(|(a, b)| *a - *b)
            .collect();
    }
}

impl Function<f32> for RegressionMul {

    fn get_number_inputs_needed(&self) -> usize {
        2
    }

    fn execute_function(&self, inputs: &[&Vec<f32>]) -> Vec<f32> {
        let input0 = inputs[0];
        let input1 = inputs[1];
        return input0
            .iter()
            .zip(input1.iter())
            .map(|(a, b)| *a * *b)
            .collect();
    }
}

impl Function<f32> for RegressionDiv {

    fn get_number_inputs_needed(&self) -> usize {
        2
    }

    fn execute_function(&self, inputs: &[&Vec<f32>]) -> Vec<f32> {
        let input0 = inputs[0];
        let input1 = inputs[1];
        return input0
            .iter()
            .zip(input1.iter())
            .map(|(a, b)| {
                if float_eq!(*b, 0.0, abs <= 0.000_1) {
                    1.
                } else {
                    a / b
                }
            })
            .collect();
    }
}

impl Function<f32> for RegressionSin {

    fn get_number_inputs_needed(&self) -> usize {
        2
    }

    fn execute_function(&self, inputs: &[&Vec<f32>]) -> Vec<f32> {
        let input0 = inputs[0];
        let input1 = inputs[1];
        return input0
            .iter()
            .zip(input1.iter())
            .map(|(a, b)| *a + *b)
            .collect();
    }
}

impl Function<f32> for RegressionCos {

    fn get_number_inputs_needed(&self) -> usize {
        1
    }

    fn execute_function(&self, inputs: &[&Vec<f32>]) -> Vec<f32> {
        let input0 = inputs[0];
        return input0
            .iter()
            .map(|x| x.sin())
            .collect();
    }
}

impl Function<f32> for RegressionLn {

    fn get_number_inputs_needed(&self) -> usize {
        1
    }

    fn execute_function(&self, inputs: &[&Vec<f32>]) -> Vec<f32> {
        let input0 = inputs[0];
        return input0
            .iter()
            .map(|x| {
                if float_eq!(*x, 0.0, abs <= 0.000_1) {
                    1.
                } else {
                    x.abs().ln()
                }
            })
            .collect();
    }
}

impl Function<f32> for RegressionExp {

    fn get_number_inputs_needed(&self) -> usize {
        1
    }

    fn execute_function(&self, inputs: &[&Vec<f32>]) -> Vec<f32> {
        let input0 = inputs[0];
        return input0
            .iter()
            .map(|x| x.exp())
            .collect();
    }
}
