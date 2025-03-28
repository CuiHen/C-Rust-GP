use std::iter::Iterator;

use rand;
use rand::prelude::SliceRandom;
use rand::thread_rng;


pub struct CrossValidation {
    fold_indices: Vec<Vec<usize>>,
    current_fold: usize,
}


impl CrossValidation {
    pub fn new(n_samples: usize, n_folds: usize) -> Self {
        let mut indices: Vec<usize> = (0..n_samples).collect();
        let mut rng = thread_rng();

        indices.shuffle(&mut rng);

        let samples_per_fold = (n_samples as f32 / n_folds as f32).floor() as usize;


        let mut fold_indices: Vec<Vec<usize>> = vec![];

        for i in 0..n_folds {
            let mut temp_indices = indices[i * samples_per_fold..(i + 1) * samples_per_fold].to_vec();
            temp_indices.sort_unstable();
            temp_indices.reverse();
            fold_indices.push(temp_indices);
        }

        // sort fold indices and rev

        Self {
            fold_indices,
            current_fold: 0,
        }
    }

    pub fn split(&mut self, mut dataset: Vec<Vec<f32>>, mut label: Vec<usize>)
                 -> (Vec<Vec<f32>>, Vec<usize>, Vec<Vec<f32>>, Vec<usize>) {
        let indices = &self.fold_indices[self.current_fold];

        let mut new_test_data: Vec<Vec<f32>> = vec![];
        let mut new_test_labels: Vec<usize> = vec![];

        for index in indices {
            new_test_data.push(dataset.swap_remove(*index));
            new_test_labels.push(label.swap_remove(*index));
        }

        self.current_fold += 1;

        (dataset, label, new_test_data, new_test_labels)
    }
}

