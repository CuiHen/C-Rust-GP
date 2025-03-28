pub fn fitness_regression(prediction: &Vec<Vec<f32>>, labels: &Vec<Vec<f32>>) -> f32 {
    assert_eq!(prediction.len(), labels.len());
    assert_eq!(prediction[0].len(), labels[0].len());
    let mut fitness: f32 = 0.;
    prediction.iter().zip(labels.iter()).for_each(|(inner_pred, inner_label)|
            inner_pred.iter().zip(inner_label.iter()).for_each(|(x, y)| fitness += (x - y).abs())
    );

    fitness /= prediction.len() as f32;

    if fitness.is_nan() {
        fitness = f32::MAX;
    } else if fitness.is_infinite() {
        fitness = f32::MAX;
    }

    fitness
}


pub fn fitness_boolean(prediction: &Vec<Vec<bool>>, labels: &Vec<Vec<bool>>) -> f32 {
    assert_eq!(prediction.len(), labels.len());

    let mut fitness: i32 = 0;
    prediction.iter().zip(labels.iter()).for_each(|(inner_pred, inner_label)|
        inner_pred.iter().zip(inner_label.iter()).for_each(|(x, y)| { if x == y { fitness += 1 } })
    );

    let number_bits = labels[0].len() * labels.len();

    
    1. - (fitness as f32 / number_bits as f32)
}