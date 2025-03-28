use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::components::cgp_components::cgp_types::CGPType;
use crate::components::evo_operators_for_population::crossover_operators::crossover_types::CrossoverType;
use crate::components::evo_operators_for_population::mutation_operators::mutation_types::MutationTypes;
use crate::components::evo_operators_for_population::selection_operators::selection_types::SelectionTypes;

#[derive(Clone, Serialize, Deserialize)]
pub struct CgpParameters {
    // General CGP related parameters

    pub cgp_type: CGPType,
    pub graph_width: usize,
    pub nbr_inputs: usize,
    pub nbr_outputs: usize,
    pub number_functions: usize,
    pub fitness_threshold: f32,

    // selection related parameters
    pub selection_type: SelectionTypes,
    // important:
    // case 1: (mu + lambda)-ES: elitists == mu
    // case 2: tournament: elitists == elitists, independent of population size
    pub elitists: usize,
    // important:
    // case 1: (mu + lambda)-ES: population-size == lambda
    // case 2: tournament: population-size
    pub population_size: usize,
    pub tournament_size: usize,

    // mutation related parameters
    pub mutation_type: MutationTypes,
    pub multi_n_mutations: usize,
    pub split_mutation_rate_active: f32,
    pub split_mutation_rate_inactive: f32,
    pub mutation_rate: f32,

    // Crossover related parameters
    pub crossover_type: CrossoverType,
    pub crossover_rate: f32,
    pub crossover_multi_n: usize,

}


impl Display for CgpParameters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        writeln!(f, "\"CGP_Type\": \"{}\",", self.cgp_type)?;
        writeln!(f, "\"nbr_nodes\": \"{}\",", self.graph_width)?;

        writeln!(f, "\"selection_type\": \"{}\",", self.selection_type)?;
        writeln!(f, "\"elitists\": \"{}\",", self.elitists)?;
        writeln!(f, "\"population_size\": \"{}\",", self.population_size)?;
        writeln!(f, "\"tournament_size\": \"{}\",", self.tournament_size)?;

        writeln!(f, "\"mutation_type\": \"{}\",", self.mutation_type)?;
        writeln!(f, "\"nbr_mutations_(for_multi-n)\": \"{}\",", self.multi_n_mutations)?;
        writeln!(f, "\"mutation_rate\": \"{}\",", self.mutation_rate)?;
        writeln!(f, "\"mutation_rate_active\": \"{}\",", self.split_mutation_rate_active)?;
        writeln!(f, "\"mutation_rate_inactive\": \"{}\",", self.split_mutation_rate_inactive)?;

        writeln!(f, "\"crossover_type\": \"{}\",", self.crossover_type)?;
        writeln!(f, "\"crossover_rate\": \"{}\",", self.crossover_rate)?;
        writeln!(f, "\"crossover_multi_n\": \"{}\"", self.crossover_multi_n)?;

        writeln!(f, "}}")
    }
}