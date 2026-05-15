use anyhow::{bail, Result};
use clap::{Parser, Subcommand, ValueEnum};
use crop_core::{
    bench_manifest_json, bench_pack, crop_context, eval_competitor_json, eval_fixture, eval_suite,
    fixture_graph, ingest_path, list_views, prefix_cached_pack, run_view_json,
    score_graph_for_task, CropOptions, EvalCase, EvalMethod, PrefixCacheProfile,
};
use std::fs;
use std::path::Path;
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
        /// Emit prefix-cache-aware JSON instead of a bare crop pack.
        #[arg(long, value_enum)]
        prefix_cache: Option<PrefixCacheArg>,
    },
    /// Run deterministic apple-to-apple retrieval evaluations.
    Eval {
        /// Run every fixture case against every local method.
        #[arg(long)]
        all: bool,
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
    /// Score saved external competitor retrieval results against fixture labels.
    EvalImport {
        /// JSON file using the competitor result contract.
        #[arg(long)]
        input: PathBuf,
    },
    /// Benchmark ingest, scoring, and crop stages on a local root.
    Bench {
        /// JSON manifest with repeatable benchmark cases.
        #[arg(long)]
        manifest: Option<PathBuf>,
        /// Root directory or file to benchmark.
        #[arg(long)]
        root: Option<PathBuf>,
        /// Task prompt used to score evidence units.
        #[arg(long)]
        task: Option<String>,
        /// Token budget for the context crop.
        #[arg(long, default_value_t = 12000)]
        budget: usize,
        /// Seed unit index for crop expansion.
        #[arg(long, default_value_t = 0)]
        seed: usize,
    },
    /// Run a persisted named context view recipe.
    View {
        /// List view recipes in a store directory.
        #[arg(long)]
        list: bool,
        /// JSON file using the crop.view.v1 recipe schema.
        #[arg(long)]
        file: Option<PathBuf>,
        /// View store directory for --list. Defaults to .crop\views.
        #[arg(long, default_value = ".crop\\views")]
        dir: PathBuf,
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
    LexicalTopK,
    HybridTopK,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum PrefixCacheArg {
    Generic,
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
            prefix_cache,
        } => {
            let mut ingested = ingest_path(root)?;
            score_graph_for_task(&mut ingested.graph, &task);
            let options = CropOptions {
                seed,
                token_budget: budget,
            };
            let pack = crop_context(&ingested.graph, options.clone())?;
            let json = if let Some(prefix_cache) = prefix_cache {
                serde_json::to_string_pretty(&prefix_cached_pack(
                    &ingested,
                    pack,
                    task,
                    options,
                    prefix_cache.into(),
                )?)?
            } else {
                serde_json::to_string_pretty(&pack)?
            };
            if let Some(output) = output {
                fs::write(output, json)?;
            } else {
                println!("{json}");
            }
        }
        Commands::Eval {
            all,
            case,
            method,
            budget,
        } => {
            let json = if all {
                serde_json::to_string_pretty(&eval_suite(budget)?)?
            } else {
                serde_json::to_string_pretty(&eval_fixture(case.into(), method.into(), budget)?)?
            };
            println!("{json}");
        }
        Commands::EvalImport { input } => {
            let json = fs::read_to_string(input)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&eval_competitor_json(&json)?)?
            );
        }
        Commands::Bench {
            manifest,
            root,
            task,
            budget,
            seed,
        } => {
            let json = if let Some(manifest) = manifest {
                let input = fs::read_to_string(&manifest)?;
                let base_dir = manifest.parent().unwrap_or_else(|| Path::new("."));
                serde_json::to_string_pretty(&bench_manifest_json(&input, base_dir)?)?
            } else {
                let Some(root) = root else {
                    bail!("--root is required unless --manifest is provided");
                };
                let Some(task) = task else {
                    bail!("--task is required unless --manifest is provided");
                };
                serde_json::to_string_pretty(&bench_pack(
                    root,
                    &task,
                    CropOptions {
                        seed,
                        token_budget: budget,
                    },
                )?)?
            };
            println!("{json}");
        }
        Commands::View { list, file, dir } => {
            let json = if list {
                serde_json::to_string_pretty(&list_views(dir)?)?
            } else {
                let Some(file) = file else {
                    bail!("--file is required unless --list is provided");
                };
                let input = fs::read_to_string(&file)?;
                let base_dir = file.parent().unwrap_or_else(|| Path::new("."));
                serde_json::to_string_pretty(&run_view_json(&input, base_dir)?)?
            };
            println!("{json}");
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
            EvalMethodArg::LexicalTopK => EvalMethod::LexicalTopK,
            EvalMethodArg::HybridTopK => EvalMethod::HybridTopK,
        }
    }
}

impl From<PrefixCacheArg> for PrefixCacheProfile {
    fn from(value: PrefixCacheArg) -> Self {
        match value {
            PrefixCacheArg::Generic => PrefixCacheProfile::Generic,
        }
    }
}
