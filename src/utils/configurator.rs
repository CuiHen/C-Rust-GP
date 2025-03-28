use std::rc::Rc;
use crate::components::cgp_components::cgp_node_mutation_operators::{NodeMutationDAG, NodeMutationOperator, NodeMutationGeneral};
use crate::components::cgp_components::cgp_types::CGPType;
use crate::components::cgp_components::chromosome_evaluator_operators::{ChromosomeEvaluatorGeneral, ChromosomeEvaluation};
use crate::components::cgp_components::chromosome_find_active_node_operators::{ChromosomeActiveNode, ChromosomeFindActiveNodesDAG, ChromosomeFindActiveNodesGeneral};
use crate::components::cgp_components::chromosome_mutation_operators::{ChromosomeMutationMultiN, ChromosomeMutationPoint, ChromosomeMutationSingle, ChromosomeMutationSplit, ChromosomeMutation};
use crate::components::evo_operators_for_population::crossover_operators::crossover_algos::{Crossover, PopulationMultiPointCrossover, PopulationNoCrossover, PopulationSinglePointCrossover, PopulationUniformCrossover};
use crate::components::evo_operators_for_population::crossover_operators::crossover_mulambda_elitist::PopulationCrossoverMuLambdaElitist;
use crate::components::evo_operators_for_population::crossover_operators::crossover_tournament::PopulationCrossoverTournament;
use crate::components::evo_operators_for_population::crossover_operators::crossover_trait::PopulationGeneralCrossover;
use crate::components::evo_operators_for_population::crossover_operators::crossover_types::CrossoverType;
use crate::components::evo_operators_for_population::evaluation_operators::eval_population_general::{PopulationForwardPassGeneral};
use crate::components::evo_operators_for_population::evaluation_operators::eval_population_oneplusfour::{PopulationForwardPassOnePlusFour};
use crate::components::evo_operators_for_population::evaluation_operators::eval_population_trait::{PopulationGeneralForwardPass};
use crate::components::evo_operators_for_population::mutation_operators::mutation_types::MutationTypes;
use crate::components::evo_operators_for_population::selection_operators::elitist_selection_mupluslambda::PopulationElitistSelectionMuPlusLambda;
use crate::components::evo_operators_for_population::selection_operators::elitist_selection_oneplusfour::PopulationElitistSelectionOnePlusFour;
use crate::components::evo_operators_for_population::selection_operators::elitist_selection_tournament::PopulationElitistSelectionWithTournament;
use crate::components::evo_operators_for_population::selection_operators::selection_trait::PopulationGeneralSelection;
use crate::components::evo_operators_for_population::selection_operators::selection_types::SelectionTypes;
use crate::global_params::CgpParameters;
use crate::utils::cli_functions::Cli;

pub fn make_cgp_params(args: &Cli, nbr_inputs: usize, nbr_outputs: usize, number_functions: usize) -> CgpParameters {
    let cgp_type = match args.cgp_extension_type.as_str() {
        "Standard" => CGPType::Standard,
        "OriginalReorder" => CGPType::OriginalReorder,
        "EReorder" => CGPType::EReorder,
        "LSDReorder" => CGPType::LSDReorder,
        "NegBiasReorder" => CGPType::NegBiasReorder,
        "UniformReorder" => CGPType::UniformReorder,
        "DAG" => CGPType::DAG,
        _ => panic!("Unsupported cgp type"),
    };

    let fitness_threshold = match args.dataset_args.dataset_type.as_str() {
        "bool" => 0.000_1,
        "f32" => 0.01,
        _ => { panic!("Unsupported dataset type") }
    };

    let selection_type = match args.selection_args.selection_type.as_str() {
        "OnePlusFour" => SelectionTypes::OnePlusFour,
        "MuPlusLambda" => SelectionTypes::MuPlusLambda,
        "Tournament" => SelectionTypes::Tournament,
        _ => { panic!("Unsupported selection type") }
    };
    check_selection_args_validity(args);

    let mutation_type = match args.mutation_args.mutation_type.as_str() {
        "Point" => MutationTypes::Point,
        "Single" => MutationTypes::Single,
        "Split" => MutationTypes::Split,
        "Multi" => MutationTypes::Multi,
        _ => { panic!("Unsupported mutation type") }
    };

    let crossover_type = match args.crossover_args.crossover_type.as_str() {
        "1-Point" => CrossoverType::SinglePointCrossover,
        "n-Point" => CrossoverType::MultiPointCrossover,
        "Uniform" => CrossoverType::UniformCrossover,
        "NoCrossover" => CrossoverType::NoCrossover,
        _ => { panic!("Unsupported mutation type") }
    };


    

    CgpParameters {
        // General CGP related parameters
        cgp_type,
        graph_width: args.nbr_nodes,
        nbr_inputs,
        nbr_outputs,
        number_functions,
        fitness_threshold,

        // selection related parameters
        selection_type,
        elitists: args.selection_args.elitism_number,
        population_size: args.selection_args.population_size,  // total pop-number: #elitsts + pop-size
        tournament_size: args.selection_args.tournament_size,

        // mutation related parameters
        mutation_type,
        multi_n_mutations: args.mutation_args.bioma_mutation_multi_n,
        split_mutation_rate_active: args.mutation_args.bioma_mutation_prob_active,
        split_mutation_rate_inactive: args.mutation_args.bioma_mutation_prob_inactive,
        mutation_rate: args.mutation_args.bioma_mutation_rate,

        // Crossover related parameters
        crossover_type,
        crossover_rate: args.crossover_args.crossover_rate,
        crossover_multi_n: args.crossover_args.multi_point_n,
    }
}

fn check_selection_args_validity(args: &Cli) {
    match args.selection_args.selection_type.as_str() {
        "OnePlusFour" => {
            assert_eq!(1, args.selection_args.elitism_number, "For (1+4)-ES, elitism-number must be 1");
            assert_eq!(4, args.selection_args.population_size, "For (1+4)-ES, population-size must be 4");
        }
        _ => {}
    };
}


pub fn get_node_mutation_operator(params: &CgpParameters) -> Box<dyn NodeMutationOperator> {
    if params.cgp_type == CGPType::DAG {
        return NodeMutationDAG::new();
    }

    NodeMutationGeneral::new()
}

pub fn get_active_node_finder_operator<T: Clone>(params: &CgpParameters) -> Box<dyn ChromosomeActiveNode<T>> {
    if params.cgp_type == CGPType::DAG {
        return ChromosomeFindActiveNodesDAG::new();
    }

    ChromosomeFindActiveNodesGeneral::new()
}

pub fn get_chromosome_mutation_operator(params: &CgpParameters) -> Box<dyn ChromosomeMutation> {
    match params.mutation_type {
        MutationTypes::Point => ChromosomeMutationPoint::new(),
        MutationTypes::Single => ChromosomeMutationSingle::new(),
        MutationTypes::Split => ChromosomeMutationSplit::new(),
        MutationTypes::Multi => ChromosomeMutationMultiN::new(),
    }
}

pub fn get_population_evaluator_operator<T: Clone>(params: &CgpParameters) -> Box<dyn PopulationGeneralForwardPass<T>>
where
    ChromosomeEvaluatorGeneral: ChromosomeEvaluation<T>,
{
    if params.selection_type == SelectionTypes::OnePlusFour {
        return PopulationForwardPassOnePlusFour::new();
    }
    PopulationForwardPassGeneral::new()
}

pub fn get_population_selection_operator<T: Clone>(params: &CgpParameters) -> Box<dyn PopulationGeneralSelection<T>> {
    match params.selection_type {
        SelectionTypes::OnePlusFour => PopulationElitistSelectionOnePlusFour::new(),
        SelectionTypes::MuPlusLambda => PopulationElitistSelectionMuPlusLambda::new(),
        SelectionTypes::Tournament => PopulationElitistSelectionWithTournament::new(),
    }
}

pub fn get_population_crossover_operator<T: Clone>(params: &CgpParameters) -> (Box<dyn PopulationGeneralCrossover<T>>, Rc<Box<dyn Crossover<T>>>)

{
    let crossover_population_mechanism = match params.selection_type {
        SelectionTypes::OnePlusFour => panic!("Wrong Selection type for crossover"),
        SelectionTypes::MuPlusLambda => PopulationCrossoverMuLambdaElitist::new(),
        SelectionTypes::Tournament => PopulationCrossoverTournament::new(),
    };

    let crossover_operator = match params.crossover_type {
        CrossoverType::SinglePointCrossover => PopulationSinglePointCrossover::new(),
        CrossoverType::MultiPointCrossover => PopulationMultiPointCrossover::new(),
        CrossoverType::UniformCrossover => PopulationUniformCrossover::new(),
        CrossoverType::NoCrossover => PopulationNoCrossover::new(),
    };

    (crossover_population_mechanism, Rc::new(crossover_operator))
}
