use crate::CropError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofArtifactManifest {
    pub schema_version: String,
    pub generated_by: String,
    pub config_root: PathBuf,
    pub generated_at_ms: u64,
    pub artifacts: Vec<ProofArtifactRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofArtifactRecord {
    pub source_path: PathBuf,
    pub output_path: PathBuf,
    pub target: String,
    pub status: String,
    pub directives_resolved: usize,
    pub from_cache: bool,
    pub diagnostics: Vec<ProofArtifactDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofArtifactDiagnostic {
    pub code: String,
    pub severity: String,
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofArtifactReportEntry {
    pub source: String,
    pub output: String,
    pub target: String,
    pub status: String,
    pub output_exists: bool,
    pub stale: bool,
    pub directives_resolved: usize,
    pub from_cache: bool,
    pub diagnostic_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofArtifactReport {
    pub schema_version: String,
    pub manifest: String,
    pub generated_by: String,
    pub artifact_count: usize,
    pub ok_count: usize,
    pub missing_output_count: usize,
    pub stale_count: usize,
    pub diagnostic_count: usize,
    pub artifacts: Vec<ProofArtifactReportEntry>,
}

pub fn proof_artifacts_for_root(root: impl AsRef<Path>) -> Result<ProofArtifactReport, CropError> {
    proof_artifacts(root.as_ref().join(".proof").join("artifacts.json"))
}

pub fn proof_artifacts(manifest: impl AsRef<Path>) -> Result<ProofArtifactReport, CropError> {
    let manifest = manifest.as_ref();
    if !manifest.exists() {
        return Err(CropError::RootDoesNotExist {
            path: manifest.display().to_string(),
        });
    }
    let json = fs::read_to_string(manifest).map_err(|source| CropError::ReadFile {
        path: manifest.display().to_string(),
        source,
    })?;
    proof_artifacts_json(&json, manifest)
}

pub fn proof_artifacts_json(
    json: &str,
    manifest_path: impl AsRef<Path>,
) -> Result<ProofArtifactReport, CropError> {
    let manifest: ProofArtifactManifest =
        serde_json::from_str(json).map_err(CropError::ArtifactManifestImport)?;
    let manifest_path = manifest_path.as_ref();
    let base = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    let root = if manifest.config_root.is_absolute() {
        manifest.config_root.clone()
    } else {
        base.join(&manifest.config_root)
    };

    let mut artifacts = Vec::new();
    for artifact in manifest.artifacts {
        let source_path = resolve_artifact_path(&root, &artifact.source_path);
        let output_path = resolve_artifact_path(&root, &artifact.output_path);
        let output_exists = output_path.exists();
        let stale = output_exists
            && source_path.exists()
            && file_mtime_ms(&source_path)? > file_mtime_ms(&output_path)?;
        artifacts.push(ProofArtifactReportEntry {
            source: display_path(&artifact.source_path),
            output: display_path(&artifact.output_path),
            target: artifact.target,
            status: artifact.status,
            output_exists,
            stale,
            directives_resolved: artifact.directives_resolved,
            from_cache: artifact.from_cache,
            diagnostic_count: artifact.diagnostics.len(),
        });
    }

    let missing_output_count = artifacts
        .iter()
        .filter(|artifact| !artifact.output_exists)
        .count();
    let stale_count = artifacts.iter().filter(|artifact| artifact.stale).count();
    let diagnostic_count = artifacts
        .iter()
        .map(|artifact| artifact.diagnostic_count)
        .sum();
    let ok_count = artifacts
        .len()
        .saturating_sub(missing_output_count + stale_count);
    Ok(ProofArtifactReport {
        schema_version: "crop.proof-artifacts.v1".to_string(),
        manifest: display_path(manifest_path),
        generated_by: manifest.generated_by,
        artifact_count: artifacts.len(),
        ok_count,
        missing_output_count,
        stale_count,
        diagnostic_count,
        artifacts,
    })
}

fn resolve_artifact_path(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

fn file_mtime_ms(path: &Path) -> Result<u64, CropError> {
    let modified = fs::metadata(path)
        .map_err(|source| CropError::InspectPath {
            path: path.display().to_string(),
            source,
        })?
        .modified()
        .map_err(|source| CropError::InspectPath {
            path: path.display().to_string(),
            source,
        })?;
    Ok(modified
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64)
}

fn display_path(path: &Path) -> String {
    let rendered = path.display().to_string();
    rendered
        .strip_prefix(r"\\?\")
        .unwrap_or(&rendered)
        .to_string()
}
