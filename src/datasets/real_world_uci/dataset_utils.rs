use std::collections::HashMap;
use nohash_hasher::BuildNoHashHasher;
use rand::{Rng, thread_rng};
use rand::prelude::{IteratorRandom, SliceRandom};
use crate::utils::utility_funcs::transpose;
use statrs::distribution::Normal;
use rand::distributions::{Distribution, Uniform};

#[derive(Clone, Copy)]
pub enum RedundancyType {
    Copy,
    Noise,
    NoisyCopy,
}

pub fn add_redundancies(train_data: Vec<Vec<f32>>, test_data: Option<Vec<Vec<f32>>>, bloat: f32, r_type: RedundancyType) -> (Vec<Vec<f32>>, Option<Vec<Vec<f32>>>, HashMap<usize, Vec<usize>, BuildNoHashHasher<usize>>) {
    // mapping:
    // indices_from_which_values_are_copied -> List of indices into which this index is cloned into
    let mut indice_insert_copy_mapping: HashMap<usize, Vec<usize>, BuildNoHashHasher<usize>> = HashMap::default();

    if bloat <= 0.0 {
        return (train_data, test_data, indice_insert_copy_mapping);
    }

    // tranpose for easier pushing of redundancies
    let mut train_data = transpose(train_data);
    let mut test_data = test_data.map(transpose);

    if test_data.is_some() {
        assert_eq!(train_data.len(), test_data.as_ref().unwrap().len());
    }

    let nbr_additionals: usize = ((train_data.len() as f32) * bloat).ceil() as usize;
    let mut rng = thread_rng();

    match r_type {
        RedundancyType::Copy => {
            // clone data to insert them without having to check for changed indices
            let og_train_data = train_data.clone();
            let og_test_data = test_data.clone();

            // indices from which redundancies are added
            // they are the same for both train and test data
            let mut indices_from_og_dataset: Vec<usize> = Vec::with_capacity(nbr_additionals);
            let choices: Vec<usize> = (0..train_data.len()).collect();
            for _ in 0..nbr_additionals {
                // choose index from og dataset
                let index = *choices.choose(&mut rng).unwrap();
                indices_from_og_dataset.push(index);

                indice_insert_copy_mapping.insert(index, vec![]);
            }

            // insert redundancies
            for redundancy_index in indices_from_og_dataset {
                let input_index = (0..train_data.len()).choose(&mut rng).unwrap();

                indice_insert_copy_mapping.get_mut(&redundancy_index).unwrap().push(input_index);
                // indices must be updated. if an input_index is smaller than an already input index,
                // bigger indices get moved by one, as the attribute is also moved to the right
                indice_insert_copy_mapping = indice_insert_copy_mapping.iter_mut().map(|(key, val)| {
                    for i in 0..val.len() {
                        if val[i] >= input_index {
                            val[i] += 1;
                        }
                    }
                    (*key, val.clone())
                }).collect();

                train_data.insert(input_index, og_train_data[redundancy_index].clone());
                if test_data.is_some() {
                    test_data.as_mut().unwrap().insert(input_index, og_test_data.as_ref().unwrap()[redundancy_index].clone());
                }
            }
        }
        RedundancyType::NoisyCopy => {
            // clone data to insert them without having to check for changed indices
            let og_train_data = train_data.clone();
            let og_test_data = test_data.clone();

            // indices from which redundancies are added
            // they are the same for both train and test data
            let mut redundancy_indices: Vec<usize> = Vec::with_capacity(nbr_additionals);
            let choices: Vec<usize> = (0..train_data.len()).collect();
            for _ in 0..nbr_additionals {
                let index = *choices.choose(&mut rng).unwrap();
                redundancy_indices.push(index);
                indice_insert_copy_mapping.insert(index, vec![]);
            }

            // instantiate distribution
            let between = Uniform::new(-0.1, 0.1);


            // insert redundancies
            for redundancy_index in redundancy_indices {
                let input_index = (0..train_data.len()).choose(&mut rng).unwrap();

                // indices must be updated. if an input_index is smaller than an already input index,
                // bigger indices get moved by one, as the attribute is also moved to the right
                indice_insert_copy_mapping = indice_insert_copy_mapping.iter_mut().map(|(key, val)| {
                    for i in 0..val.len() {
                        if val[i] >= input_index {
                            val[i] += 1;
                        }
                    }
                    (*key, val.clone())
                }).collect();
                indice_insert_copy_mapping.get_mut(&redundancy_index).unwrap().push(input_index);


                // noise data
                let train_data_to_insert = og_train_data[redundancy_index].clone();
                let random_noise_ranges: Vec<f32> = (0..train_data_to_insert.len()).map(|_| between.sample(&mut rng)).collect();
                let train_data_to_insert: Vec<f32> = train_data_to_insert
                    .iter()
                    .zip(random_noise_ranges.iter())
                    .map(|(val, noise)| val + val * noise)
                    .collect();

                train_data.insert(input_index, train_data_to_insert);
                if test_data.is_some() {
                    let test_data_to_insert = og_test_data.as_ref().unwrap()[redundancy_index].clone();
                    let test_data_to_insert: Vec<f32> = test_data_to_insert
                        .iter()
                        .zip(random_noise_ranges.iter())
                        .map(|(val, noise)| val + val * noise)
                        .collect();

                    test_data.as_mut().unwrap().insert(input_index, test_data_to_insert);
                }
            }
        }
        RedundancyType::Noise => {
            indice_insert_copy_mapping.insert(0, vec![]);

            // indices from which redundancies are added
            let nbr_noisy_values = train_data[0].len();
            // distribution with mean 0 and std 1
            let normal_distribution = Normal::new(0.0, 1.0).unwrap();

            // add noise:
            for _ in 0..nbr_additionals {
                let input_index = (0..train_data.len()).choose(&mut rng).unwrap();

                // indices must be updated. if an input_index is smaller than an already input index,
                // bigger indices get moved by one, as the attribute is also moved to the right
                indice_insert_copy_mapping = indice_insert_copy_mapping.iter_mut().map(|(key, val)| {
                    for i in 0..val.len() {
                        if val[i] >= input_index {
                            val[i] += 1;
                        }
                    }
                    (*key, val.clone())
                }).collect();
                indice_insert_copy_mapping.get_mut(&0).unwrap().push(input_index);

                // create noise vec
                let new_vec: Vec<_> = (0..nbr_noisy_values).map(|_| rng.sample(normal_distribution)).collect();
                // noise vec is f64; but data is f32 -> cast to f32
                let new_vec = new_vec.iter().map(|x| *x as f32).collect();
                train_data.insert(input_index, new_vec);

                if test_data.is_some() {
                    // create new noise vec
                    let new_vec: Vec<_> = (0..nbr_noisy_values).map(|_| rng.sample(normal_distribution)).collect();
                    // noise vec is f64; but data is f32 -> cast to f32
                    let new_vec = new_vec.iter().map(|x| *x as f32).collect();
                    test_data.as_mut().unwrap().insert(input_index, new_vec);
                }
            }
        }
    }

    let train_data = transpose(train_data);
    let test_data = test_data.map(transpose);

    (train_data, test_data, indice_insert_copy_mapping)
}


fn mean(v: &[f32]) -> f32 {
    let len = v.len() as f32;
    return v.iter().sum::<f32>() / len;
}

fn standard_deviation(v: &Vec<f32>) -> f32 {
    // calculate variance
    let len = v.len() as f32;
    let c = mean(v);
    let sum = v.iter().map(|x| (*x - c) * (*x - c)).sum::<f32>();
    let var = sum / (len - 1.0);  // unbiased estimator by applying Bessel's correction


    var.sqrt()
}

pub fn standardize_dataset(data: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let train_data_t = transpose(data);
    let mut standardized_data: Vec<Vec<f32>> = Vec::with_capacity(train_data_t.len());

    for entry_t in train_data_t {
        let mean = mean(&entry_t);
        let std = standard_deviation(&entry_t);
        let std_ized = entry_t
            .iter()
            .map(|elem| (elem - mean) / std)
            .collect::<Vec<f32>>();
        standardized_data.push(std_ized);
    }

    
    transpose(standardized_data)
}

/// Inefficient but does the trick. Shuffle data and corresponding label.
pub fn shuffle(v1: Vec<Vec<f32>>, v2: Vec<usize>) -> (Vec<Vec<f32>>, Vec<usize>) {
    assert!(v1.len() == v2.len());

    let len = v1.len();
    let mut len_range: Vec<usize> = (0..len).collect();

    len_range.shuffle(&mut thread_rng());

    let mut new1: Vec<Vec<f32>> = vec![];
    let mut new2: Vec<usize> = vec![];
    for idx in len_range {
        new1.push(v1.get(idx).unwrap().clone());
        new2.push(*v2.get(idx).unwrap());
    }

    (new1, new2)
}


pub fn preprocess_and_split(datas: Vec<Vec<f32>>, labels: Vec<usize>, standardize: bool)
                            -> (Vec<Vec<f32>>, Vec<usize>, Vec<Vec<f32>>, Vec<usize>)
{
    let datas = match standardize {
        true => { standardize_dataset(datas) }
        false => { datas }
    };

    let (datas, labels) = shuffle(datas, labels);
    let total_len = datas.len();
    let split_idx = total_len as f32 * 0.8;
    let split_idx = split_idx as usize;

    let train_data: Vec<Vec<f32>> = datas.clone()[0..split_idx].to_vec();
    let train_label: Vec<usize> = labels[0..split_idx].to_vec();
    let test_data: Vec<Vec<f32>> = datas[split_idx..total_len].to_vec();
    let test_label: Vec<usize> = labels[split_idx..total_len].to_vec();


    (train_data, train_label, test_data, test_label)
}