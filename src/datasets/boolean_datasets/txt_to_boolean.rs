use std::fs;

pub fn get_dataset(data_path: &str, label_path: &str) -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
    let content_data = fs::read_to_string(data_path).expect("Cannot read data file.");
    let content_label = fs::read_to_string(label_path).expect("Cannot read label file.");

    let mut data_vec: Vec<Vec<bool>> = Vec::new();
    for line in content_data.lines() {
        let entry: Vec<bool> = line.split(", ").map(|x| x == "true").collect();
        data_vec.push(entry);
    }

    let mut label_vec: Vec<Vec<bool>> = Vec::new();
    for line in content_label.lines() {
        let entry: Vec<bool> = line.split(", ").map(|x| x == "true").collect();
        label_vec.push(entry);
    }
    (data_vec, label_vec)
}

