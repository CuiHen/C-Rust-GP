#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fs;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use clap::Parser;
use cgp_master::function_set::boolean_function_set;
use cgp_master::function_set::regression_function_set;
use cgp_master::components::cgp_components::cgp_node::CGPNode;
use cgp_master::components::cgp_components::cgp_node_mutation_operators::*;
use cgp_master::global_params::CgpParameters;
use cgp_master::components::cgp_components::cgp_node_types::NodeType;
use cgp_master::components::cgp_components::cgp_types::CGPType;
use cgp_master::components::cgp_components::chromosome_evaluator_operators::*;
use cgp_master::components::cgp_components::chromosome_find_active_node_operators::*;
use cgp_master::components::cgp_components::chromosome_mutation_operators::*;
use cgp_master::components::cgp_components::chromosome_reorder_operators::*;
use cgp_master::components::evo_operators_for_population::crossover_operators::crossover_mulambda_elitist::CrossoverMuLambdaElitist;
use cgp_master::components::evo_operators_for_population::crossover_operators::crossover_tournament::CrossoverTournament;
use cgp_master::components::evo_operators_for_population::crossover_operators::crossover_trait::GeneralCrossoverTrait;
use cgp_master::components::evo_operators_for_population::crossover_operators::crossover_types::CrossoverType;
use cgp_master::components::evo_operators_for_population::evaluation_operators::eval_population_mupluslambda::{ForwardPassPopulationMuPlusLambda, ForwardPassPopulationTournament, TestPopulationMuPlusLambda};
use cgp_master::components::evo_operators_for_population::evaluation_operators::eval_population_oneplusfour::{EAForwardPassPopulationOnePlusFour, EATestPopulationOnePlusFour};
use cgp_master::components::evo_operators_for_population::evaluation_operators::eval_population_trait::{GeneralForwardPassPopulationTrait, GeneralTestPopulationTrait};
use cgp_master::components::evo_operators_for_population::general_operators::clone_parent_to_child::{CloneParentToChild, ClonePopulationTrait};
use cgp_master::components::evo_operators_for_population::general_operators::reorder_population::{GeneralReorderPopulationTrait, ReorderPopulation};
use cgp_master::components::evo_operators_for_population::mutation_operators::mutate_population::EAMutate;
use cgp_master::components::evo_operators_for_population::selection_operators::elitist_selection_oneplusfour::EAElitistSelectionOnePlusFour;
use cgp_master::components::evo_operators_for_population::mutation_operators::mutation_trait::GeneralMutatePopulationTrait;
use cgp_master::components::evo_operators_for_population::selection_operators::elitist_selection_mupluslambda::ElitistSelectionMuPlusLambda;
use cgp_master::components::evo_operators_for_population::selection_operators::elitist_selection_tournament::ElitistSelectionWithTournament;
use cgp_master::components::evo_operators_for_population::selection_operators::selection_trait::GeneralSelectionTrait;

use cgp_master::utils::runner::Runner;
use cgp_master::utils::utility_funcs;
use cgp_master::datasets::boolean_datasets;
use cgp_master::datasets::regression_benchmarks;

use cgp_master::utils::txt_writer::*;
use cgp_master::function_set::function_trait::FunctionTrait;
fn standard_loop() {
    // transpose so a whole row of the dataset can be used as an array for calculation
    // let (data, label) = boolean_datasets::parity::get_dataset();
    let (data, label) = boolean_datasets::encode::get_dataset();
    // let (data, label) = regression_benchmarks::koza_3::get_dataset();
    // let (eval_data, eval_label) = regression_benchmarks::koza_3::get_eval_dataset();
    // let (data, label) = regression_benchmarks::keijzer::get_dataset();
    // let (eval_data, eval_label) = regression_benchmarks::keijzer::get_eval_dataset();
    //
    // let function_set = regression_function_set::get_regression_function_set();
    let function_set = boolean_function_set::get_boolean_function_set();

    // let fitness_threshold = 0.01;  // Regression
    let fitness_threshold = 0.0001;  // Bool

    let node_mutation_op = Rc::new(NodeMutationStandard::new());
    let chromosome_mutation_op = Rc::new(ChromosomeMutationSingle::new());
    let chromosome_active_op = Rc::new(ChromosomeFindActiveNodesStandard::new());
    let chromosome_eval_op = Rc::new(ChromosomeEvaluator::new());

    let clone_parent2child = CloneParentToChild::new();
    let mutation_operator = EAMutate::new();
    let eval_operator = EAForwardPassPopulationOnePlusFour::new();
    let selection_operator = EAElitistSelectionOnePlusFour::new();
    // let test_operator = TestPopulationOnePlusFour::new();


    let mut params = CgpParameters {
        cgp_type: CGPType::Standard,
        graph_width: 100,
        elitists: 1,
        population_size: 4,
        eval_after_iterations: 0,
        nbr_inputs: data[0].len(),
        nbr_outputs: label[0].len(),
        mutation_rate: 0.0,
        crossover_type: CrossoverType::NoCrossover,
        crossover_rate: 0.0,
        multi_point_n: 0,
        tournament_size: 0,
        number_functions: function_set.len(),
        fitness_threshold,
    };


    // let mut runner = Runner::new(params, data, label, Some(eval_data), Some(eval_label), Rc::clone(&function_set), Rc::clone(&chromosome_active_op));
    let mut runner = Runner::new(params, data, label, None, None, Rc::clone(&function_set), Rc::clone(&chromosome_active_op));

    for i in 0..100_000 {
        if i % 500 == 0 {
            println!("i: {}, fitness: {}", i, runner.get_best_fitness());
        }

        clone_parent2child.execute(&mut runner);
        mutation_operator.execute(&mut runner, Rc::clone(&node_mutation_op), Rc::clone(&chromosome_mutation_op));
        eval_operator.execute(&mut runner, Rc::clone(&chromosome_eval_op), Rc::clone(&chromosome_active_op), Rc::clone(&function_set));
        selection_operator.execute(&mut runner);

        if runner.get_best_fitness() < fitness_threshold {
            println!("{}", i);
            break;
        }
    }
    // let test_fit: f32 = test_operator.execute(&mut runner, Rc::clone(&chromosome_eval_op), Rc::clone(&chromosome_active_op), Rc::clone(&function_set));
    // println!("Final Train Fitness: {}", runner.get_best_fitness());
    // println!("Final Test Fitness: {}", test_fit);
}