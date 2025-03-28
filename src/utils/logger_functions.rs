use std::fs;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use crate::components::cgp_components::chromosome_find_active_node_operators::ChromosomeActiveNode;
use crate::function_set::function_trait::Function;
use crate::utils::runner::{get_best_parent_chromosome, ProgramState};
use std::io::{BufWriter, Write};
use crate::global_params::CgpParameters;
use crate::utils::cli_functions::Cli;

pub struct LoggerActiveNodes {
    file: BufWriter<File>,
}
pub struct LoggerFitness {
    file: BufWriter<File>,
}

impl LoggerActiveNodes {
    pub fn new(args: &Cli, params: &CgpParameters) -> Self {
        let save_path = Path::new("")
            .join(format!("Experiments_Output_{}", args.dataset_args.dataset_type))
            .join(format!("cgp_extension_type_{}", args.cgp_extension_type))
            .join(format!("dataset_{}", args.dataset_args.dataset))
            .join(format!("mutation_type_{}", args.mutation_args.mutation_type))
            .join(format!("selection_type_{}", args.selection_args.selection_type))
            .join(format!("crossover_type_{}", args.crossover_args.crossover_type))
            .join(format!("run_id_{}", args.run_id));

        fs::create_dir_all(save_path.clone()).unwrap();
        let save_file_iteration = "active_nodes.txt".to_string();
        let file = BufWriter::new(File::create(save_path.join(save_file_iteration))
            .expect("cannot create file"));

        // if no parameter file exisists, create one
        let parameter_file_path = "parameters.txt".to_string();
        let parameter_file_path = save_path.join(parameter_file_path);
        if !parameter_file_path.exists() {
            let mut params_file = File::create(parameter_file_path).expect("cannot create file");
            writeln!(params_file, "{}", params).expect("cannot write to file");
        }

        Self {
            file,
        }
    }

    pub fn write_active_nodes<T>(&mut self,
                                 runner: &mut ProgramState<T>,
                                 active_node_func: Rc<Box<dyn ChromosomeActiveNode<T>>>,
                                 function_set: Rc<Vec<Box<dyn Function<T>>>>)
    {
        let mut parent = get_best_parent_chromosome(runner);

        active_node_func.execute(&mut parent, Rc::clone(&function_set));

        write!(self.file, "{:?}", parent.active_nodes).expect("cannot write");
    }
}

impl LoggerFitness {
    pub fn new(args: &Cli, params: &CgpParameters) -> Self {
        let save_path = Path::new("")
            .join(format!("Experiments_Output_{}", args.dataset_args.dataset_type))
            .join(format!("cgp_extension_type_{}", args.cgp_extension_type))
            .join(format!("dataset_{}", args.dataset_args.dataset))
            .join(format!("mutation_type_{}", args.mutation_args.mutation_type))
            .join(format!("selection_type_{}", args.selection_args.selection_type))
            .join(format!("crossover_type_{}", args.crossover_args.crossover_type))
            .join(format!("run_id_{}", args.run_id));

        fs::create_dir_all(save_path.clone()).unwrap();

        let save_file_iteration = "fitness.txt".to_string();
        let file = BufWriter::new(File::create(save_path.join(save_file_iteration))
            .expect("cannot create file"));

        // if no parameter file exisists, create one
        let parameter_file_path = "parameters.txt".to_string();
        let parameter_file_path = save_path.join(parameter_file_path);
        if !parameter_file_path.exists() {
            let mut params_file = File::create(parameter_file_path).expect("cannot create file");
            writeln!(params_file, "{}", params).expect("cannot write to file");
        }

        Self {
            file,
        }
    }

    pub fn write_fitness(&mut self, iteration_number: usize, fitness: f32) {
        writeln!(self.file, "Iteration: {iteration_number}, Fitness: {:?}", fitness).expect("write not okay??");
    }

    pub fn write_finished_fitness(&mut self, iteration_number: usize, fitness_train: Option<f32>, fitness_test: Option<f32>) {
        writeln!(self.file, "End at iteration: {}", iteration_number).expect("cannot write");

        if fitness_train.is_some() {
            writeln!(self.file, "Fitness Train: {}", fitness_train.unwrap()).expect("cannot write");
        }
        if fitness_test.is_some() {
            writeln!(self.file, "Fitness Eval: {}", fitness_test.unwrap()).expect("cannot write");
        }
    }
}



