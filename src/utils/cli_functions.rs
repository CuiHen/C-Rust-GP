use clap::{Args, Parser};

#[derive(Parser)]
#[clap(author, version, about, name = "User Args")]
pub struct Cli {
    #[arg(long,
        default_value = "Standard",
        help = "Default options include:
        - Standard,
        - OriginalReorder,
        - EReorder,
        - LSDReorder,
        - NegBiasReorder,
        - UniformReorder,
        - DAG
        ")]
    pub cgp_extension_type: String,

    #[arg(long, default_value_t = 50)]
    pub nbr_nodes: usize,

    #[arg(
        long,
        default_value_t = 0,
        help = "Helper value to differentiate multiple runs of the same configuration."
    )]
    pub run_id: usize,

    // #[arg(
    //     long,
    //     default_value = "",
    //     help = "Optional value. If the configuration is derived from a json file, include the path here."
    // )]
    // pub json_file_path: String,

    #[command(flatten)]
    pub dataset_args: DatasetArgs,

    #[command(flatten)]
    pub mutation_args: MutationArgs,

    #[command(flatten)]
    pub crossover_args: CrossoverArgs,

    #[command(flatten)]
    pub selection_args: SelectionArgs,
}

#[derive(Args)]
#[group(required = false)]
pub struct DatasetArgs {
    #[arg(long,
        default_value = "f32",
        help = "Default options: f32 or bool")]
    pub dataset_type: String,

    #[arg(long,
        default_value = "abalone",
        help = "Default options include:
        - Parity,
        - Encode,
        - Decode,
        - Multiply,
        - Keijzer6,
        - Koza3,
        - Nguyen7,
        - Pagie1
        ")]
    pub dataset: String,
}

#[derive(Args)]
#[group(required = false)]
pub struct MutationArgs {
    #[arg(long,
        default_value = "Single",
        help = "Default options include:
        - Point,
        - Single,
        - Split,
        - Multi,
        ")]
    pub mutation_type: String,

    #[arg(long, default_value_t = 0)]
    pub bioma_mutation_multi_n: usize,

    #[arg(long, default_value_t = 0.0)]
    pub bioma_mutation_prob_active: f32,

    #[arg(long, default_value_t = 0.0)]
    pub bioma_mutation_prob_inactive: f32,

    #[arg(long, default_value_t = 0.0)]
    pub bioma_mutation_rate: f32,
}


#[derive(Args)]
#[group(required = false)]
pub struct CrossoverArgs {
    #[arg(long, default_value = "Uniform",
        help = "Default options include:
        - 1-Point,
        - n-Point,
        - Uniform,
        - NoCrossover,
        In case of n-Point crossover: n must be defined with variable multi-point-n
        ")]
    pub crossover_type: String,

    #[arg(long, default_value_t = 0.9)]
    pub crossover_rate: f32,

    // for n-point crossover
    #[arg(long, default_value_t = 0)]
    pub multi_point_n: usize,
}

#[derive(Args)]
#[group(required = false)]
pub struct SelectionArgs {
    #[arg(long, default_value = "OnePlusFour",
        help = "Default options include:
        - OnePlusFour,
        - MuPlusLambda,
        - Tournament (includes elitists; tournament draws from both elitists and normal population),
        ")]
    pub selection_type: String,

    #[arg(long, default_value_t = 8, help = "Only relevant if selection_type=Tournament")]
    pub tournament_size: usize,

    #[arg(long, default_value_t = 1, help = "Relevant for MuPlusLambda and Tournament Selection.
    Case MuPlusLambda: Mu==Elitism-Number
    Case Tournament: Number of elitists -> Total Population = Elitism-Number + Population-size
    ")]
    pub elitism_number: usize,

    #[arg(long, default_value_t = 4, help = "Relevant for MuPlusLambda and Tournament Selection.
    Case MuPlusLambda: Lambda==Population-size
    Case Tournament: Population size -> Total Population = Elitism-Number + Population-size
    ")]
    pub population_size: usize,

}

pub fn get_arguments() -> Cli {
    

    Cli::parse()
}
