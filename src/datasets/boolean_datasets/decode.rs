pub fn get_dataset() -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
    let data = [
        [false, false, false, false],
        [false, false, false, true],
        [false, false, true, false],
        [false, false, true, true],
        [false, true, false, false],
        [false, true, false, true],
        [false, true, true, false],
        [false, true, true, true],
        [true, false, false, false],
        [true, false, false, true],
        [true, false, true, false],
        [true, false, true, true],
        [true, true, false, false],
        [true, true, false, true],
        [true, true, true, false],
        [true, true, true, true], ];

    let labels = [
        [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false],
        [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false],
        [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false],
        [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false],
        [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false],
        [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false],
        [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false],
        [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false],
        [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true], ];

    let mut data_vec: Vec<Vec<bool>> = Vec::new();
    for d in data {
        let temp = Vec::from(d);
        data_vec.push(temp);
    }

    let mut label_vec: Vec<Vec<bool>> = Vec::new();
    for d in labels {
        let temp = Vec::from(d);
        label_vec.push(temp);
    }

    (data_vec, label_vec)
}

