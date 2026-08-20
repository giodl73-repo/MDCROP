use anyhow::{bail, Result};
use clap::ValueEnum;
use mdcrop_core::{proof_artifacts, proof_artifacts_for_root, render_proof_artifact_markdown};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ArtifactFormatArg {
    Json,
    Markdown,
}

pub fn run(
    root: Option<PathBuf>,
    manifest: Option<PathBuf>,
    format: ArtifactFormatArg,
    output: Option<PathBuf>,
) -> Result<()> {
    if root.is_some() && manifest.is_some() {
        bail!("--root and --manifest cannot be combined");
    }
    let report = if let Some(manifest) = manifest {
        proof_artifacts(manifest)?
    } else {
        let Some(root) = root else {
            bail!("--root or --manifest is required");
        };
        proof_artifacts_for_root(root)?
    };
    let rendered = match format {
        ArtifactFormatArg::Json => serde_json::to_string_pretty(&report)?,
        ArtifactFormatArg::Markdown => render_proof_artifact_markdown(&report),
    };
    if let Some(output) = output {
        fs::write(output, rendered)?;
    } else {
        println!("{rendered}");
    }
    Ok(())
}
