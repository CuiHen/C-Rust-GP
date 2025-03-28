use crate::utils::utility_funcs::get_float_iterator;

fn make_label(inputs: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut labels: Vec<f32> = vec![];
    for d in inputs {
        labels.push(d.iter().map(|x| 1. / x).sum::<f32>());
    }

    vec![labels]
}


pub fn get_dataset() -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    let mut data = vec![];

    for x in get_float_iterator(1., 50., 1.) {
        let mut elem: Vec<f32> = vec![];
        elem.push(x);

        data.push(elem);
    }

    let labels = make_label(&data);

    (data, labels)
}

pub fn get_eval_dataset() -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    let mut data = vec![];

    for x in get_float_iterator(1., 120., 1.) {
        let mut elem: Vec<f32> = vec![];
        elem.push(x);

        data.push(elem);
    }


    let labels = make_label(&data);

    (data, labels)
}


