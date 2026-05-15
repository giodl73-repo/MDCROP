use anyhow::Result;
use clap::{Parser, Subcommand};
use crop_core::{crop_context, CropOptions, EvidenceGraph, EvidenceUnit};

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
    }
    Ok(())
}

fn fixture_graph() -> EvidenceGraph {
    EvidenceGraph {
        units: vec![
            EvidenceUnit {
                id: "task-seed".to_string(),
                source_id: "fixture:task".to_string(),
                relevance: 1.0,
                token_estimate: 100,
            },
            EvidenceUnit {
                id: "bridge-note".to_string(),
                source_id: "fixture:note".to_string(),
                relevance: 0.9,
                token_estimate: 50,
            },
            EvidenceUnit {
                id: "stale-noise".to_string(),
                source_id: "fixture:old".to_string(),
                relevance: 0.1,
                token_estimate: 50,
            },
        ],
        adjacency: vec![vec![1, 2], vec![0], vec![0]],
    }
}
