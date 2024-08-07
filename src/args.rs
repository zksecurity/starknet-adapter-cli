pub use crate::prover;

use clap::{Args, Parser, ValueHint};
use prover::config::{ProverConfig, ProverParametersConfig};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "stone-cli",
    version = "0.1.0-alpha",
    about = "CLI for proving Cairo 1 programs on Starknet"
)]
#[command(bin_name = "stone-cli")]
#[allow(clippy::large_enum_variant)]
pub enum Cli {
    Prove(ProveArgs),
    Verify(VerifyArgs),
}

#[derive(Args, Debug)]
#[command(version)]
pub struct ProveArgs {
    #[clap(long = "cairo_program", value_hint=ValueHint::FilePath)]
    pub cairo_program: PathBuf,

    #[clap(
        long = "program_input",
        help = "Arguments should be spaced, with array elements placed between brackets, e.g. '1 2 [1 2 3]'"
    )]
    pub program_input: Option<String>,

    #[clap(long = "program_input_file", value_hint=ValueHint::FilePath, conflicts_with="program_input")]
    pub program_input_file: Option<PathBuf>,

    #[clap(long = "layout", default_value = "recursive", value_enum)]
    pub layout: LayoutName,

    #[clap(long = "prover_config_file", conflicts_with_all = ["store_full_lde", "use_fft_for_eval", "constraint_polynomial_task_size", "n_out_of_memory_merkle_layers", "table_prover_n_tasks_per_segment"])]
    pub prover_config_file: Option<PathBuf>,

    #[clap(long = "parameter_file", conflicts_with_all = ["field", "channel_hash", "commitment_hash", "n_verifier_friendly_commitment_layers", "pow_hash", "page_hash", "fri_step_list", "last_layer_degree_bound", "n_queries", "proof_of_work_bits", "log_n_cosets", "use_extension_field", "verifier_friendly_channel_updates", "verifier_friendly_commitment_hash"])]
    pub parameter_file: Option<PathBuf>,

    #[clap(long = "output", default_value = "./proof.json")]
    pub output: PathBuf,

    #[clap(flatten)]
    pub parameter_config: ProverParametersConfig,

    #[clap(flatten)]
    pub prover_config: ProverConfig,
}

#[derive(Args, Debug)]
pub struct VerifyArgs {
    #[clap(long = "proof", value_parser)]
    pub proof: PathBuf,
}
/// Enum representing the name of a Cairo Layout
#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq, Hash, clap::ValueEnum)]
#[allow(non_camel_case_types)]
pub enum LayoutName {
    plain,
    small,
    dex,
    recursive,
    starknet,
    starknet_with_keccak,
    recursive_large_output,
    recursive_with_poseidon,
    all_solidity,
    all_cairo,
    dynamic,
}

impl LayoutName {
    pub fn to_str(self) -> &'static str {
        match self {
            LayoutName::plain => "plain",
            LayoutName::small => "small",
            LayoutName::dex => "dex",
            LayoutName::recursive => "recursive",
            LayoutName::starknet => "starknet",
            LayoutName::starknet_with_keccak => "starknet_with_keccak",
            LayoutName::recursive_large_output => "recursive_large_output",
            LayoutName::recursive_with_poseidon => "recursive_with_poseidon",
            LayoutName::all_solidity => "all_solidity",
            LayoutName::all_cairo => "all_cairo",
            LayoutName::dynamic => "all_cairo",
        }
    }
}

impl std::str::FromStr for LayoutName {
    type Err = ();

    fn from_str(layout: &str) -> Result<Self, Self::Err> {
        match layout {
            "plain" => Ok(LayoutName::plain),
            "small" => Ok(LayoutName::small),
            "dex" => Ok(LayoutName::dex),
            "recursive" => Ok(LayoutName::recursive),
            "starknet" => Ok(LayoutName::starknet),
            "starknet_with_keccak" => Ok(LayoutName::starknet_with_keccak),
            "recursive_large_output" => Ok(LayoutName::recursive_large_output),
            "recursive_with_poseidon" => Ok(LayoutName::recursive_with_poseidon),
            "all_solidity" => Ok(LayoutName::all_solidity),
            "all_cairo" => Ok(LayoutName::all_cairo),
            "dynamic" => Ok(LayoutName::dynamic),
            _ => Err(()),
        }
    }
}
