pub trait FitnessFunction<P> {
    fn fitness_function(prediction: &Vec<P>, labels: &Vec<P>) -> f32;
}