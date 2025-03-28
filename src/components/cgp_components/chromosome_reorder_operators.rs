use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use nohash_hasher::BuildNoHashHasher;
use rand::distributions::{Distribution, Uniform};
use statrs::distribution::Beta;
use rand::prelude::IteratorRandom;
use rand::thread_rng;
use crate::utils::linspace::linspace;
use crate::components::cgp_components::cgp_node::CGPNode;
use crate::components::cgp_components::chromosome::Chromosome;
use crate::components::cgp_components::chromosome_find_active_node_operators::ChromosomeActiveNode;
use crate::function_set::function_trait::Function;
use crate::utils::utility_funcs;

pub trait ChromosomeReorder<T> {
    fn new() -> Box<dyn ChromosomeReorder<T>> where Self: Sized;
    fn execute(&self,
               chromosome: &mut Chromosome,
               active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>,
               function_set: Rc<Vec<Box<dyn Function<T>>>>);

    /// Needed for every Reorder class but `ChromosomeReorderStandard`
    /// Updates the connection of a reordered CGP genotype.
    fn update_connections(&self,
                          new_nodes_grid: &mut Vec<CGPNode>,
                          node_id: usize,
                          swapped_pos_indices: &mut HashMap<usize,
                              usize,
                              BuildNoHashHasher<usize>>) {
        let con1 = new_nodes_grid[node_id].connection0;
        let con2 = new_nodes_grid[node_id].connection1;

        new_nodes_grid[node_id].connection0 = *swapped_pos_indices.get(&con1)
            .unwrap_or(&con1);
        new_nodes_grid[node_id].connection1 = *swapped_pos_indices.get(&con2)
            .unwrap_or(&con2);
    }

    /// Needed for every Reorder class but `ChromosomeReorderStandard`
    /// Returns a Vec of active nodes which only contains computational nodes.
    fn private_get_active_computational_nodes(&self, chromosome: &Chromosome) -> Option<Vec<usize>> {
        // clone active nodes and only take active computational nodes:
        let mut c_active_nodes: Vec<usize> = chromosome.active_nodes.clone();

        // remove output nodes
        // Note: Because remove shifts the vec over the remaining elements, it has a worst-case
        // performance of O(n).
        // Thus, reverse the output indices to minimize shifting.
        for output_node_id in (
            (chromosome.params.nbr_inputs + chromosome.params.graph_width)
                ..
                (chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs)
        ).rev() {
            let index = c_active_nodes
                .iter()
                .position(|x| *x == output_node_id)
                .unwrap();
            c_active_nodes.remove(index);
        }

        // remove input nodes, as only computational nodes are going to be swapped
        // Note: Because remove shifts the vec over the remaining elements, it has a worst-case
        // performance of O(n).
        // Thus, reverse the output indices to minimize shifting.
        for input_node_id in (0..chromosome.params.nbr_inputs).rev() {
            let index = c_active_nodes
                .iter()
                .position(|x| *x == input_node_id);
            if let Some(idx) = index {
                c_active_nodes.remove(idx);
            }
        }

        if c_active_nodes.is_empty() {
            return None;
        }

        Some(c_active_nodes)
    }

    /// Needed for every Reorder class but `ChromosomeReorderStandard`
    /// Reorders the genome of a chromosome based on the given new positions.
    fn private_reorder(&self,
                       chromosome: &mut Chromosome,
                       c_active_nodes: &Vec<usize>,
                       new_pos_active: &Vec<usize>,
                       active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>,
                       function_set: Rc<Vec<Box<dyn Function<T>>>>,
    ) {
        let comp_nodes_ids: Vec<usize> = (chromosome.params.nbr_inputs..(chromosome.params.nbr_inputs + chromosome.params.graph_width)).collect();
        let mut old_pos_inactive = utility_funcs::vect_difference(&comp_nodes_ids, c_active_nodes);
        let mut new_pos_inactive = utility_funcs::vect_difference(&comp_nodes_ids, new_pos_active);

        old_pos_inactive.sort_unstable();
        new_pos_inactive.sort_unstable();

        assert_eq!(c_active_nodes.len(), new_pos_active.len());
        assert_eq!(old_pos_inactive.len(),
                   new_pos_inactive.len(),
                   "actives: \n{:?} \n{:?}", c_active_nodes, new_pos_active);

        let mut swapped_pos_indices: HashMap<usize, usize, BuildNoHashHasher<usize>> = HashMap::with_capacity_and_hasher(chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs, BuildNoHashHasher::default());

        // Nodes are not swapped because that could destroy ordering
        // Instead, create a new node_list by cloning the old one
        let mut new_nodes_grid: Vec<CGPNode> = chromosome.nodes_grid.clone();

        // Input nodes are ignored, as they do not change
        // Insert active computational nodes and change their position
        for (old_node_id, new_node_id) in c_active_nodes
            .iter()
            .zip(new_pos_active.iter()) {
            let mut node = chromosome.nodes_grid[*old_node_id].clone();
            node.position = *new_node_id;

            // Case: for regression benchmarks, when connection1 is not used (i.e. node uses log func)
            // then: the node which connects to 1 is not active.
            // This is a problem for the following edge case:
            // I.e. node: position 3, connection1=2
            // gets moved to position 2. As connection1 is not acknowledged as active, former node 2
            // does  not change position. Thus, connection1 of node will not be updated. Hence, its
            // new spec is the following: position 2, connection1=2 -> this leads to a cicle
            // if this node mutates its function to something that uses two inputs (i.e. add)
            if node.connection1 >= *new_node_id {
                node.connection1 = utility_funcs::gen_random_number_for_node(node.connection1, node.position)
            }

            new_nodes_grid[*new_node_id] = node;

            swapped_pos_indices.insert(*old_node_id, *new_node_id);
        }


        // Now distribute all inactive nodes to the free indice
        for (old_node_id, new_node_id) in old_pos_inactive.iter().zip(new_pos_inactive.iter()) {
            assert!(!new_pos_active.contains(new_node_id));

            let mut node = chromosome.nodes_grid[*old_node_id].clone();
            // Here, an inactive node might need to mutate a new connection.
            // This is the case when it is connected to an active node that is now re-ordered into a
            // position that is in front of it.
            node.position = *new_node_id;
            if node.connection0 >= *new_node_id {
                node.connection0 = utility_funcs::gen_random_number_for_node(node.connection0, node.position)
            }
            if node.connection1 >= *new_node_id {
                node.connection1 = utility_funcs::gen_random_number_for_node(node.connection1, node.position)
            }
            new_nodes_grid[*new_node_id] = node;

            assert!(new_nodes_grid[*new_node_id].position > new_nodes_grid[*new_node_id].connection0, "assert 2 for node: {}", *new_node_id);
            assert!(new_nodes_grid[*new_node_id].position > new_nodes_grid[*new_node_id].connection1, "assert 3 for node: {}", *new_node_id);
        }


        // update connections of active nodes
        for node_id in new_pos_active {
            self.update_connections(&mut new_nodes_grid, *node_id, &mut swapped_pos_indices);
        }


        // update connections for output nodes
        for node_id in (chromosome.params.nbr_inputs + chromosome.params.graph_width)..(chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs) {
            // self.update_connections(&mut new_nodes_grid, node_id, &mut swapped_pos_indices);
            // ChromosomeReorderTrait::<T>::update_connections(self, &mut new_nodes_grid, node_id, &mut swapped_pos_indices);
            self.update_connections(&mut new_nodes_grid, node_id, &mut swapped_pos_indices);
        }

        chromosome.nodes_grid = new_nodes_grid;
        active_node_func.execute(chromosome, Rc::clone(&function_set));
    }
}

pub struct ChromosomeReorderStandard;

pub struct ChromosomeReorderEquidistant;

pub struct ChromosomeReorderNegativeBias;

pub struct ChromosomeReorderNormalDistribution;

pub struct ChromosomeReorderLeftSkewed;


impl<T> ChromosomeReorder<T> for ChromosomeReorderStandard {
    fn new() -> Box<dyn ChromosomeReorder<T>> where Self: Sized {
        Box::new(Self)
    }
    fn execute(&self,
               chromosome: &mut Chromosome,
               active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>,
               function_set: Rc<Vec<Box<dyn Function<T>>>>) {
        let mut rng = thread_rng();

        let total_nbr_nodes = chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs;

        // !vars with names from the original paper and its description!
        // index ; new location of the node in the grid
        let mut new_node_index = chromosome.params.nbr_inputs;
        // new_loc ; save the old location and the position of the new location
        let mut changed_locations: HashMap<usize, usize, BuildNoHashHasher<usize>> = HashMap::default();
        changed_locations.reserve(total_nbr_nodes);
        // - ; which nodes are already visited?
        let mut used_node_indices: Vec<usize> = Vec::with_capacity(total_nbr_nodes);
        // input_locations (?) ; get the index of each node with its connection-id. input-node-id is removed
        let mut node_dependencies: HashMap<usize, Vec<usize>, BuildNoHashHasher<usize>> = self.determine_node_dependency(chromosome);
        // addable ; addable nodes
        let mut addable: Vec<usize> = vec![];
        self.get_addable(&mut node_dependencies, &mut addable);

        while !addable.is_empty() {
            // current_node_id is also the position of the node in the grid
            let i = (0..addable.len()).choose(&mut rng).unwrap();
            let current_node_id = addable.swap_remove(i);

            // map old location to new location
            changed_locations.insert(current_node_id, new_node_index);

            // update dependencies
            // check if the current node id exists in each dependency entry. if exists, remove
            for val in node_dependencies.values_mut() {
                val.retain(|&x| x != current_node_id);
            }
            // update params
            new_node_index += 1;
            used_node_indices.push(current_node_id);
            self.get_addable(&mut node_dependencies, &mut addable);
        }

        self.update_node_index(chromosome, &changed_locations);
        self.update_node_connections(chromosome, &changed_locations);

        active_node_func.execute(chromosome, Rc::clone(&function_set));

        assert_eq!(changed_locations.len(), chromosome.params.graph_width);
        assert_eq!(used_node_indices.len(), chromosome.params.graph_width);
    }
}

impl ChromosomeReorderStandard {
    fn determine_node_dependency(&self, chromosome: &mut Chromosome) -> HashMap<usize, Vec<usize>, BuildNoHashHasher<usize>> {
        // Get list with connections of each computational node.
        let mut node_dependencies: HashMap<usize, Vec<usize>, BuildNoHashHasher<usize>> = HashMap::default();

        // init hashmaps with input nodes - but they are removed later!
        // Input nodes do not have dependencies - they only propagate the input.
        // for node_index in 0..(chromosome.params.nbr_inputs + chromosome.params.graph_width) {
        for node_index in chromosome.params.nbr_inputs..(chromosome.params.nbr_inputs + chromosome.params.graph_width) {
            node_dependencies.insert(node_index, Vec::with_capacity(2));
        }

        // iterate through each computational node and get their connections
        for node_index in chromosome.params.nbr_inputs..(chromosome.params.nbr_inputs + chromosome.params.graph_width) {
            let current_node = &chromosome.nodes_grid[node_index];
            let con1 = current_node.connection0;
            let con2 = current_node.connection1;

            // check each time if con1 / con2 are input nodes !
            // if con1 or con2 not input nodes; add them
            if !(0..chromosome.params.nbr_inputs).contains(&con1) {
                node_dependencies
                    .get_mut(&node_index)
                    .unwrap()
                    .push(con1)
            }

            if !(0..chromosome.params.nbr_inputs).contains(&con2) {
                node_dependencies
                    .get_mut(&node_index)
                    .unwrap()
                    .push(con2)
            }
        }


        node_dependencies
    }

    fn get_addable(&self, node_dependencies: &mut HashMap<usize, Vec<usize>, BuildNoHashHasher<usize>>, addable: &mut Vec<usize>) {
        // get all empty node_ids -> get all nodes which link condition is satisfied
        let temp_addable: Vec<usize> = node_dependencies
            .iter()
            .filter(|(_, y)| y.is_empty())
            .map(|(&x, _)| x)
            .collect();
        if !temp_addable.is_empty() {
            addable.extend(temp_addable);
        }

        // node_dependencies.retain(|_, v| *v != (Vec::<usize>::from([])));
        node_dependencies.retain(|_, v| !v.is_empty());
    }

    fn update_node_index(&self, chromosome: &mut Chromosome, location_mapping: &HashMap<usize, usize, BuildNoHashHasher<usize>>) {
        // Nodes are not swapped because that could destroy ordering
        // Instead, create a new node_list by cloning the old one
        let mut new_nodes_grid: Vec<CGPNode> = chromosome.nodes_grid.clone();
        // Input and output nodes are ignored, as they do not change
        // Insert computational nodes
        for (old_position, new_position) in location_mapping.iter() {
            let mut node = chromosome.nodes_grid[*old_position].clone();
            node.position = *new_position;

            new_nodes_grid[*new_position] = node;
        }

        chromosome.nodes_grid = new_nodes_grid;
    }

    fn update_node_connections(&self, chromosome: &mut Chromosome, location_mapping: &HashMap<usize, usize, BuildNoHashHasher<usize>>) {
        let total_nbr_nodes = chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs;

        for node_id in chromosome.params.nbr_inputs..total_nbr_nodes {
            let node = &mut chromosome.nodes_grid[node_id];

            if location_mapping.get(&node.connection0).is_some() {
                node.connection0 = *location_mapping.get(&node.connection0).unwrap();
            }
            if location_mapping.get(&node.connection1).is_some() {
                node.connection1 = *location_mapping.get(&node.connection1).unwrap();
            }
        }
    }
}


impl<T> ChromosomeReorder<T> for ChromosomeReorderEquidistant {
    fn new() -> Box<dyn ChromosomeReorder<T>> where Self: Sized {
        Box::new(Self)
    }
    fn execute(&self,
               chromosome: &mut Chromosome,
               active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>,
               function_set: Rc<Vec<Box<dyn Function<T>>>>) {

        // Get active computational nodes
        let c_active_nodes: Option<Vec<usize>> = ChromosomeReorder::<T>::private_get_active_computational_nodes(self, chromosome);
        if c_active_nodes.is_none() {
            return;
        }
        let c_active_nodes = c_active_nodes.unwrap();

        let new_pos_active: Vec<usize> = linspace(chromosome.params.nbr_inputs,
                                                  chromosome.params.nbr_inputs + chromosome.params.graph_width - 1,
                                                  c_active_nodes.len());

        ChromosomeReorder::<T>::private_reorder(self,
                                                chromosome,
                                                &c_active_nodes,
                                                &new_pos_active,
                                                Rc::clone(&active_node_func),
                                                Rc::clone(&function_set));

        // let comp_nodes_ids: Vec<usize> = (chromosome.params.nbr_inputs..(chromosome.params.nbr_inputs + chromosome.params.graph_width)).collect();
        // let mut old_pos_inactive = utility_funcs::vect_difference(&comp_nodes_ids, &c_active_nodes);
        // let mut new_pos_inactive = utility_funcs::vect_difference(&comp_nodes_ids, &new_pos_active);
        //
        // old_pos_inactive.sort_unstable();
        // new_pos_inactive.sort_unstable();
        //
        // assert_eq!(c_active_nodes.len(), new_pos_active.len());
        // assert_eq!(old_pos_inactive.len(),
        //            new_pos_inactive.len(),
        //            "actives: \n{:?} \n{:?}", c_active_nodes, new_pos_active);
        //
        // let mut swapped_pos_indices: HashMap<usize, usize, BuildNoHashHasher<usize>> = HashMap::with_capacity_and_hasher(chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs, BuildNoHashHasher::default());
        //
        //
        // // Nodes are not swapped because that could destroy ordering
        // // Instead, create a new node_list by cloning the old one
        // let mut new_nodes_grid: Vec<CGPNode> = chromosome.nodes_grid.clone();
        //
        // // Input nodes are ignored, as they do not change
        // // Insert active computational nodes and change their position
        // for (old_node_id, new_node_id) in c_active_nodes
        //     .iter()
        //     .zip(new_pos_active.iter()) {
        //
        //     let mut node = chromosome.nodes_grid[*old_node_id].clone();
        //     node.position = *new_node_id;
        //     new_nodes_grid[*new_node_id] = node;
        //
        //     swapped_pos_indices.insert(*old_node_id, *new_node_id);
        // }
        //
        // // Now distribute all inactive nodes to the free indice
        // for (old_node_id, new_node_id) in old_pos_inactive.iter().zip(new_pos_inactive.iter()) {
        //     assert!(!new_pos_active.contains(new_node_id));
        //
        //     let mut node = chromosome.nodes_grid[*old_node_id].clone();
        //     // Here, an inactive node might need to mutate a new connection.
        //     // This is the case when it is connected to an active node that is now re-ordered into a
        //     // position that is in front of it.
        //     if node.connection0 >= *new_node_id {
        //         node.connection0 = utility_funcs::gen_random_number_for_node(node.connection0, *new_node_id)
        //     }
        //     if node.connection1 >= *new_node_id {
        //         node.connection1 = utility_funcs::gen_random_number_for_node(node.connection1, *new_node_id)
        //     }
        //     node.position = *new_node_id;
        //     new_nodes_grid[*new_node_id] = node;
        //
        //     assert!(new_nodes_grid[*new_node_id].position > new_nodes_grid[*new_node_id].connection0, "assert 2 for node: {}", *new_node_id);
        //     assert!(new_nodes_grid[*new_node_id].position > new_nodes_grid[*new_node_id].connection1, "assert 3 for node: {}", *new_node_id);
        // }
        //
        //
        // // update connections of active nodes
        // for node_id in &new_pos_active {
        //     ChromosomeReorderTrait::<T>::update_connections(self, &mut new_nodes_grid, *node_id, &mut swapped_pos_indices);
        //     // self.update_connections(&mut new_nodes_grid, *node_id, &mut swapped_pos_indices);
        // }
        //
        // // update connections for output nodes
        // for node_id in (chromosome.params.nbr_inputs + chromosome.params.graph_width)..(chromosome.params.nbr_inputs + chromosome.params.graph_width + chromosome.params.nbr_outputs) {
        //     // self.update_connections(&mut new_nodes_grid, node_id, &mut swapped_pos_indices);
        //     ChromosomeReorderTrait::<T>::update_connections(self, &mut new_nodes_grid, node_id, &mut swapped_pos_indices);
        //
        // }
        //
        // chromosome.nodes_grid = new_nodes_grid;
        // active_node_func.execute(chromosome, Rc::clone(&function_set));
    }
}

impl<T> ChromosomeReorder<T> for ChromosomeReorderNegativeBias {
    fn new() -> Box<dyn ChromosomeReorder<T>> where Self: Sized {
        Box::new(Self)
    }
    fn execute(&self,
               chromosome: &mut Chromosome,
               active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>,
               function_set: Rc<Vec<Box<dyn Function<T>>>>) {
        // Get active computational nodes
        let c_active_nodes: Option<Vec<usize>> = ChromosomeReorder::<T>::private_get_active_computational_nodes(self, chromosome);
        if c_active_nodes.is_none() {
            return;
        }
        let c_active_nodes = c_active_nodes.unwrap();

        let new_pos_active: Vec<usize> = (chromosome.params.nbr_inputs + chromosome.params.graph_width - c_active_nodes.len()
            ..
            chromosome.params.nbr_inputs + chromosome.params.graph_width).collect();

        ChromosomeReorder::<T>::private_reorder(self,
                                                chromosome,
                                                &c_active_nodes,
                                                &new_pos_active,
                                                Rc::clone(&active_node_func),
                                                Rc::clone(&function_set));
    }
}

impl<T> ChromosomeReorder<T> for ChromosomeReorderNormalDistribution {
    fn new() -> Box<dyn ChromosomeReorder<T>> where Self: Sized {
        Box::new(Self)
    }
    fn execute(&self,
               chromosome: &mut Chromosome,
               active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>,
               function_set: Rc<Vec<Box<dyn Function<T>>>>) {
        let mut rng = thread_rng();

        // Get active computational nodes
        let c_active_nodes: Option<Vec<usize>> = ChromosomeReorder::<T>::private_get_active_computational_nodes(self, chromosome);
        if c_active_nodes.is_none() {
            return;
        }
        let c_active_nodes = c_active_nodes.unwrap();

        // sample new uniformly distributed values
        // must be hashset at first to prevent duplicates while generating numbers
        let mut new_pos_active: HashSet<usize, BuildNoHashHasher<usize>> = HashSet::default();
        new_pos_active.reserve(c_active_nodes.len());

        let between = Uniform::from(chromosome.params.nbr_inputs..(chromosome.params.nbr_inputs + chromosome.params.graph_width - 1));
        // cannot draw c_active_nodes.len() random sample points because they could overlap.
        // Hence: Draw into set until desired number of nodes
        loop {
            new_pos_active.insert(between.sample(&mut rng));
            if new_pos_active.len() == c_active_nodes.len() {
                break;
            }
        }

        let mut new_pos_active: Vec<usize> = new_pos_active.into_iter().collect();
        new_pos_active.sort_unstable();

        ChromosomeReorder::<T>::private_reorder(self,
                                                chromosome,
                                                &c_active_nodes,
                                                &new_pos_active,
                                                Rc::clone(&active_node_func),
                                                Rc::clone(&function_set));
    }
}

impl<T> ChromosomeReorder<T> for ChromosomeReorderLeftSkewed {
    fn new() -> Box<dyn ChromosomeReorder<T>> where Self: Sized {
        Box::new(Self)
    }
    fn execute(&self,
               chromosome: &mut Chromosome,
               active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>,
               function_set: Rc<Vec<Box<dyn Function<T>>>>) {
        let mut rng = thread_rng();

        // Get active computational nodes
        let c_active_nodes: Option<Vec<usize>> = ChromosomeReorder::<T>::private_get_active_computational_nodes(self, chromosome);
        if c_active_nodes.is_none() {
            return;
        }
        let c_active_nodes = c_active_nodes.unwrap();

        // sample new left-skewed distributed values
        // use beta distribution for this
        // must be hashset at first to prevent duplicates while generating numbers
        let mut new_pos_active: HashSet<usize, BuildNoHashHasher<usize>> = HashSet::default();
        new_pos_active.reserve(c_active_nodes.len());

        // samples between 0. and 1.
        let beta_dis = Beta::new(6.0, 1.0).unwrap();
        // because of the float range between 0 and 1, it must be converted to usize node positions
        // do: multiplicate with nbr of nodes -> gets value in [0, nbr_nodes]
        // add nbr of input nodes to shift value -> [nbr_input, nbr_input + nbr_nodes]
        let scale_factor: f64 = chromosome.params.graph_width as f64;
        let shift_factor: f64 = chromosome.params.nbr_inputs as f64;

        // cannot draw c_active_nodes.len() random sample points because they could overlap.
        // Hence: Draw into set until desired number of nodes
        loop {
            // generate new value and shift it
            let mut new_sample: f64 = beta_dis.sample(&mut rng);
            new_sample *= scale_factor;
            new_sample += shift_factor;

            new_pos_active.insert(new_sample as usize);
            if new_pos_active.len() == c_active_nodes.len() {
                break;
            }
        }

        let mut new_pos_active: Vec<usize> = new_pos_active.into_iter().collect();
        new_pos_active.sort_unstable();

        ChromosomeReorder::<T>::private_reorder(self,
                                                chromosome,
                                                &c_active_nodes,
                                                &new_pos_active,
                                                Rc::clone(&active_node_func),
                                                Rc::clone(&function_set));
    }
}
