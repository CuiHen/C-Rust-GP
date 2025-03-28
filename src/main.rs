#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;
use std::rc::Rc;
use std::time::Instant;
use cgp_master::function_set::boolean_function_set;
use cgp_master::function_set::regression_function_set;
use cgp_master::components::cgp_components::cgp_node_mutation_operators::*;
use cgp_master::global_params::CgpParameters;
use cgp_master::components::cgp_components::cgp_types::CGPType;
use cgp_master::components::cgp_components::chromosome_evaluator_operators::*;
use cgp_master::components::cgp_components::chromosome_find_active_node_operators::*;
use cgp_master::components::cgp_components::chromosome_mutation_operators::*;
use cgp_master::components::cgp_components::chromosome_reorder_operators::*;
use cgp_master::components::evo_operators_for_population::crossover_operators::crossover_trait::PopulationGeneralCrossover;
use cgp_master::components::evo_operators_for_population::crossover_operators::crossover_types::CrossoverType;
use cgp_master::components::evo_operators_for_population::evaluation_operators::eval_population_oneplusfour::PopulationForwardPassOnePlusFour;
use cgp_master::components::evo_operators_for_population::evaluation_operators::eval_population_trait::PopulationGeneralForwardPass;
use cgp_master::components::evo_operators_for_population::general_operators::clone_parent_to_child::{CloneParentToChild, ClonePopulation};
use cgp_master::components::evo_operators_for_population::general_operators::reorder_population::GeneralReorderPopulationTrait;
use cgp_master::components::evo_operators_for_population::mutation_operators::mutate_population_general::PopulationMutationGeneral;
use cgp_master::components::evo_operators_for_population::selection_operators::elitist_selection_oneplusfour::PopulationElitistSelectionOnePlusFour;
use cgp_master::components::evo_operators_for_population::mutation_operators::mutation_trait::PopulationGeneralMutation;
use cgp_master::components::evo_operators_for_population::selection_operators::selection_trait::PopulationGeneralSelection;

use cgp_master::utils::runner::ProgramState;
use cgp_master::utils::configurator::*;
use cgp_master::datasets::boolean_datasets;
use cgp_master::datasets::regression_benchmarks;
use cgp_master::datasets::real_world_uci;
use cgp_master::utils::cli_functions::{get_arguments, Cli};
use cgp_master::utils::logger_functions::{LoggerActiveNodes, LoggerFitness};
use cgp_master::utils::checkpoint::*;


fn standard_bool(args: Cli) {
    let (data, label) = match args.dataset_args.dataset.as_str() {
        "Parity" => boolean_datasets::parity::get_dataset(),
        "Encode" => boolean_datasets::encode::get_dataset(),
        "Decode" => boolean_datasets::decode::get_dataset(),
        "Multiply" => boolean_datasets::multiply::get_dataset(),
        _ => { panic!("Unknown dataset type {}", args.dataset_args.dataset_type) }
    };

    let function_set = boolean_function_set::get_boolean_function_set();

    let params = make_cgp_params(&args, data[0].len(), label[0].len(), function_set.len());

    let node_mutation_op = Rc::new(get_node_mutation_operator(&params));
    let chromosome_active_op = Rc::new(get_active_node_finder_operator(&params));

    let chromosome_mutation_op: Rc<Box<dyn ChromosomeMutation>> = Rc::new(get_chromosome_mutation_operator(&params));

    let chromosome_eval_op = Rc::new(ChromosomeEvaluatorGeneral::new());

    let clone_parent2child = CloneParentToChild::new();
    let mut mutation_operator = PopulationMutationGeneral::new();
    let eval_operator = get_population_evaluator_operator(&params);
    let selection_operator = get_population_selection_operator(&params);

    let mut runner = ProgramState::new(params.clone(), data, label, None, None, Rc::clone(&function_set), Rc::clone(&chromosome_eval_op), Rc::clone(&chromosome_active_op));

    let mut logger_fitness = LoggerFitness::new(&args, &params);
    let mut logger_active_nodes = LoggerActiveNodes::new(&args, &params);

    let save_path = Path::new("")
        .join(format!("Experiments_Output_{}", args.dataset_args.dataset_type))
        .join(format!("cgp_extension_type_{}", args.cgp_extension_type))
        .join(format!("dataset_{}", args.dataset_args.dataset))
        .join(format!("mutation_type_{}", args.mutation_args.mutation_type))
        .join(format!("selection_type_{}", args.selection_args.selection_type))
        .join(format!("crossover_type_{}", args.crossover_args.crossover_type))
        .join(format!("run_id_{}", args.run_id));
    let save_file_iteration = format!("mutated_nodes_{}.txt", args.run_id);
    let mut mutationfile = File::create(save_path.join(save_file_iteration))
        .expect("cannot create file");

    let mut iteration_number = 0;

    for i in 0..500_000 {
        logger_fitness.write_fitness(iteration_number, runner.get_best_fitness());
        if i % 500 == 0 {
            println!("i: {}, fitness: {}", i, runner.get_best_fitness());
        }

        iteration_number += 1;

        clone_parent2child.execute(&mut runner);
        mutation_operator.execute(&mut runner, Rc::clone(&node_mutation_op), Rc::clone(&chromosome_mutation_op));
        eval_operator.execute(&mut runner, Rc::clone(&chromosome_eval_op), Rc::clone(&chromosome_active_op), Rc::clone(&function_set));
        selection_operator.execute(&mut runner);

        if runner.get_best_fitness() < params.fitness_threshold {
            break;
        }
    }

    println!("{}", iteration_number);

    logger_fitness.write_finished_fitness(iteration_number, None, None);

    logger_active_nodes.write_active_nodes(&mut runner, Rc::clone(&chromosome_active_op), Rc::clone(&function_set));
}

fn main() {
    let args = get_arguments();
    standard_bool(args);
}
