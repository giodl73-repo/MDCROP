use anyhow::{bail, Result};
use crop_core::{proof_artifacts, proof_artifacts_for_root};
use std::fs;
use std::path::PathBuf;

pub fn run(
    root: Option<PathBuf>,
    manifest: Option<PathBuf>,
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
    let json = serde_json::to_string_pretty(&report)?;
    if let Some(output) = output {
        fs::write(output, json)?;
    } else {
        println!("{json}");
    }
    Ok(())
}
