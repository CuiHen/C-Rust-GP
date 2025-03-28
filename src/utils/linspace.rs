

pub fn linspace(x0: usize, xend: usize, n: usize) -> Vec<usize> {
    let a: Vec<f32> = itertools_num::linspace::<f32>(x0 as f32, xend as f32, n).collect();

    let x: Vec<usize> = a.iter().map(|&e| e as usize).collect();

    x

}


