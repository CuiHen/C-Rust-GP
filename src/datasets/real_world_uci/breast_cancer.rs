use std::fs;
use crate::datasets::real_world_uci::dataset_utils::{shuffle, standardize_dataset};

pub fn get_dataset(dataset_path: String, shuffle_and_split: bool) -> (Vec<Vec<f32>>,
                         Vec<usize>,
                         Vec<Vec<f32>>,
                         Vec<usize>) {
    let contents = fs::read_to_string(dataset_path)
        .expect("Should have been able to read the file");
    let contents = contents.lines();

    let mut datas: Vec<Vec<f32>> = vec![];
    let mut labels: Vec<usize> = vec![];
    for line in contents {
        let line: Vec<&str> = line.split(',').collect();

        // second entry is label
        let label;
        if line[1] == "B" {
            label = 0;
        } else if line[1]  == "M"{
            label = 1;
        } else {
            panic!("{}", line[1]);
        }
        labels.push(label);

        // first entry is ID, second label -> slice first two out
        let line = &line[2..];
        let converted_data: Vec<f32> = line.iter().map(|val| val.parse::<f32>().unwrap()).collect();
        datas.push(converted_data);
    }

    datas = standardize_dataset(datas);

    if shuffle_and_split {
        let (datas, labels) = shuffle(datas, labels);
        let total_len = datas.len();
        let split_idx = total_len as f32 * 0.8;
        let split_idx = split_idx as usize;

        let train_data: Vec<Vec<f32>> = datas.clone()[0..split_idx].to_vec();
        let train_label: Vec<usize> = labels[0..split_idx].to_vec();
        let test_data: Vec<Vec<f32>> = datas[split_idx..total_len].to_vec();
        let test_label: Vec<usize> = labels[split_idx..total_len].to_vec();

        return (train_data, train_label, test_data, test_label);

    }

    (datas, labels, vec![], vec![])
}
