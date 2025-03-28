use std::collections::HashSet;
use rand::distributions::{Distribution, Uniform};

/// Return all indices of `comp_value` in `vecs`.
pub fn get_argmins_of_value(vecs: &[f32], comp_value: f32) -> Vec<usize>{
    let mut argmins: Vec<usize> = vec![];

    vecs.iter()
        .enumerate()
        .for_each(|(i, v)| {
            if *v == comp_value {
            // Checking for _almost equal_ leads to bugs in this case (see commented out line with float_eq).
            // However, there is a finite number of values *v can take, and it is dependend on the fitness metric.
            // Thus, there should not be a case where _the same fitness_ has two different floating point 
            // values.
            // if float_eq!(*v, comp_value, abs <= 0.000_1) {
                argmins.push(i);
            }
        });

    argmins
}

pub fn get_argmin(vecs: &[f32]) -> usize {
    vecs.iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(index, _)| index)
        .unwrap()
}

pub fn get_min(vecs: &[f32]) -> f32 {
    *vecs.iter()
        .min_by(|a, b| a.partial_cmp(b)
            .unwrap())
        .unwrap()
}


pub fn vect_difference(v1: &Vec<usize>, v2: &Vec<usize>) -> Vec<usize> {
    let s1: HashSet<usize, nohash_hasher::BuildNoHashHasher<usize>> = v1.iter().cloned().collect();
    let s2: HashSet<usize, nohash_hasher::BuildNoHashHasher<usize>> = v2.iter().cloned().collect();
    (&s1 - &s2).iter().cloned().collect()
}


/// upper_range is exclusive
pub fn gen_random_number_for_node(excluded: usize, upper_range: usize) -> usize {
    // This is an important edge-case scenario.
    // In case upper_range <= 1: The node that is currently mutated has a position of 1
    // The random number generator will generate a random number in range [0, 1)
    // That means, it will always generate 0.
    // upper_range = 1 also means, that excluded will be 0. So the "newness-check" will always
    // return false. The loop will not terminate.
    // Hence, return 0 in this case.
    if upper_range <= 1 {
        return 0;
    }

    let between = Uniform::from(0..upper_range);
    let mut rng = rand::thread_rng();

    loop {
        let rand_nbr: usize = between.sample(&mut rng);
        if rand_nbr != excluded {
            return rand_nbr;
        }
    }
}

/// Transposes a Vec<Vec<T>> and returns the transposed matrix
pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn get_float_iterator(start: f32, threshold: f32, step_size: f32) -> impl Iterator<Item=f32> {
    let threshold: f32 = threshold + 1.;
    std::iter::successors(Some(start), move |&prev| {
        let next = prev + step_size;
        (next < threshold).then_some(next)
    })
}