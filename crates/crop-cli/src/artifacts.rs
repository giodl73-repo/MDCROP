use anyhow::{bail, Result};
use clap::ValueEnum;
use crop_core::{mdloom_artifacts, mdloom_artifacts_for_root, render_mdloom_artifact_markdown};
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
        mdloom_artifacts(manifest)?
    } else {
        let Some(root) = root else {
            bail!("--root or --manifest is required");
        };
        mdloom_artifacts_for_root(root)?
    };
    let rendered = match format {
        ArtifactFormatArg::Json => serde_json::to_string_pretty(&report)?,
        ArtifactFormatArg::Markdown => render_mdloom_artifact_markdown(&report),
    };
    if let Some(output) = output {
        fs::write(output, rendered)?;
    } else {
        println!("{rendered}");
    }
    Ok(())
}
