use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use crop_core::{
    crop_context, eval_fixture, fixture_graph, ingest_path, score_graph_for_task, CropOptions,
    EvalCase, EvalMethod,
};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "crop")]
#[command(about = "Graph-cut context cropping for AI agents")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Emit a deterministic fixture context pack.
    Fixture {
        /// Token budget for the fixture crop.
        #[arg(long, default_value_t = 160)]
        budget: usize,
    },
    /// Pack a local repo or document tree into a task-focused context crop.
    Pack {
        /// Root directory or file to ingest.
        #[arg(long)]
        root: PathBuf,
        /// Task prompt used to score evidence units.
        #[arg(long)]
        task: String,
        /// Token budget for the context crop.
        #[arg(long, default_value_t = 12000)]
        budget: usize,
        /// Seed unit index for crop expansion.
        #[arg(long, default_value_t = 0)]
        seed: usize,
        /// Optional output JSON path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Run deterministic apple-to-apple retrieval evaluations.
    Eval {
        /// Fixture case to evaluate.
        #[arg(long, value_enum, default_value_t = EvalCaseArg::MissingBridge)]
        case: EvalCaseArg,
        /// Retrieval method to evaluate.
        #[arg(long, value_enum, default_value_t = EvalMethodArg::Crop)]
        method: EvalMethodArg,
        /// Token budget for the evaluated context.
        #[arg(long, default_value_t = 160)]
        budget: usize,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum EvalCaseArg {
    MissingBridge,
    NoisyPeriphery,
    StaleDuplicate,
    BudgetPacking,
    CrossLinkChain,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum EvalMethodArg {
    Crop,
    ScoreTopK,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Fixture { budget } => {
            let pack = crop_context(
                &fixture_graph(),
                CropOptions {
                    seed: 0,
                    token_budget: budget,
                },
            )?;
            println!("{}", serde_json::to_string_pretty(&pack)?);
        }
        Commands::Pack {
            root,
            task,
            budget,
            seed,
            output,
        } => {
            let mut ingested = ingest_path(root)?;
            score_graph_for_task(&mut ingested.graph, &task);
            let pack = crop_context(
                &ingested.graph,
                CropOptions {
                    seed,
                    token_budget: budget,
                },
            )?;
            let json = serde_json::to_string_pretty(&pack)?;
            if let Some(output) = output {
                fs::write(output, json)?;
            } else {
                println!("{json}");
            }
        }
        Commands::Eval {
            case,
            method,
            budget,
        } => {
            let report = eval_fixture(case.into(), method.into(), budget)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
    }
    Ok(())
}

impl From<EvalCaseArg> for EvalCase {
    fn from(value: EvalCaseArg) -> Self {
        match value {
            EvalCaseArg::MissingBridge => EvalCase::MissingBridge,
            EvalCaseArg::NoisyPeriphery => EvalCase::NoisyPeriphery,
            EvalCaseArg::StaleDuplicate => EvalCase::StaleDuplicate,
            EvalCaseArg::BudgetPacking => EvalCase::BudgetPacking,
            EvalCaseArg::CrossLinkChain => EvalCase::CrossLinkChain,
        }
    }
}

impl From<EvalMethodArg> for EvalMethod {
    fn from(value: EvalMethodArg) -> Self {
        match value {
            EvalMethodArg::Crop => EvalMethod::Crop,
            EvalMethodArg::ScoreTopK => EvalMethod::ScoreTopK,
        }
    }
}
