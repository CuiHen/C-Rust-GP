use std::fs;

pub fn get_dataset(data_path: &str, label_path: &str) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    let content_data = fs::read_to_string(data_path).expect("Cannot read data file.");
    let content_label = fs::read_to_string(label_path).expect("Cannot read label file.");

    let mut data_vec: Vec<Vec<f32>> = Vec::new();
    for line in content_data.lines() {
        let entry: Vec<f32> = line.split(", ").map(|x| x.parse::<f32>().expect("Not a float")).collect();
        data_vec.push(entry);
    }

    let mut label_vec: Vec<f32> = Vec::new();
    for line in content_label.lines() {
        let entry: f32 = line.parse::<f32>().expect("Not a float");
        label_vec.push(entry);
    }
    let label_vec: Vec<Vec<f32>> = vec![label_vec];

    (data_vec, label_vec)
}
pub fn get_test_dataset(data_path: &str, label_path: &str) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    get_dataset(data_path, label_path)
}
